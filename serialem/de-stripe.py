'''
Adjust brightness of mosaic tiles so they appear seamlessly connected.

Setup:
```
git clone https://github.com/jamesra/nornir-buildmanager
(cd nornir-buildmanager && python setup.py install)
git clone https://github.com/jamesra/nornir-shared
(cd nornir-shared && python setup.py install)
```

Run examples:
~ python de-stripe.py V:\RawData\RC3 1              # de-stripe section 0001
~ python de-stripe.py V:\RawData\RC3 1,2,3          # de-stripe sections 0001, 0002, 0003
~ python de-stripe.py V:\RawData\RC3 1-21           # de-stripe all sections that exist between 0001 and 0022 (skips 0021 which is missing)
~ python de-stripe.py V:\RawData\RC3 1-21,26-30     # de-stripe multiple ranges of sections

'''

import sys
from os.path import join, exists, dirname, basename, splitext
from os import mkdir
from code import interact
from glob import glob

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.tri as mtri

import nornir_shared.plot as plot
from nornir_buildmanager.importers.idoc import IDoc, IDocTileData, ArgToIdoc, NearestLimit, SymmetricNormalize
from nornir_buildmanager.importers.serialemlog import SerialEMLog, ArgToSerialEMLog


# To be more Lisp-like, make print() return its argument.
def print_decorator(p):
    def wrapped_print(*args,**kwargs):
        p(*args,**kwargs)
        if len(args) == 1:
            return args[0]
    return wrapped_print

print = print_decorator(print)


def parseSections(arg):
    '''
    Parse a list of section numbers (as strings) from the given arg which may have comma-separated ranges and individual values
    '''
    sections = []

    if ',' in arg:
        for list_arg in arg.split(','):
            sections += parseSections(list_arg)
    else:
        if '-' in arg:
            lower, upper = [num for num in arg.split('-')]
            return [str(i) for i in range(int(lower), int(upper)+1)]
        else:
            return [arg]

    # Eliminate duplicates
    return list(set(sections))

def minMaxMeanData(section_idoc, section_log):
    '''Parse a nested list of min, max, and mean intensity data over time for the given section.
    Return value in the form required by nornir_shared.plot.PolyLine()
    '''
    # The x-axis will be the same for each line
    x_axis = [tile.startTime for tile in section_log.tileData.values()]
    return [
        # Min line
        [
            x_axis,
            [tile.Min for tile in section_idoc.tiles]
        ],
        # Max line
        [
            x_axis,
            [tile.Max for tile in section_idoc.tiles]
        ],
        # Mean line
        [
            x_axis,
            [tile.Mean for tile in section_idoc.tiles]
        ]
    ]


def whichTEM(idoc):
    if "OneView" in idoc.Note:
        return "TEM2"
    else:
        return "TEM1"

def plotIntensity(volume_dir, section):
    section_dir = join(volume_dir, section.rjust(4, "0"))

    # Some sections will be missing. Just skip them
    if not exists(section_dir):
        return

    try:
        idoc = IDoc.Load(join(section_dir, "{}.idoc".format(section)), None, False)
        log = SerialEMLog.Load(join(section_dir, "{}.log".format(section)))
    # Because some idoc files might be mis-named, do a glob if loading fails
    except:
        print("section {} has mis-named idoc/log file".format(section))
        idoc = IDoc.Load(glob(join(section_dir, '*.idoc'))[0], None, False)
        log = SerialEMLog.Load(glob(join(section_dir, "*.log.pickle"))[0].replace(".pickle", ""))

    assert idoc.NumTiles == log.NumTiles, "For section {}, the log and IDoc file have a different number of tiles!".format(section)

    scope_name = whichTEM(idoc)

    if not exists(scope_name):
        mkdir(scope_name)

    output_file = join(scope_name, "Intensity{}.svg".format(section))
    spatial_output_file = join(scope_name, "SpatialIntensity{}.svg".format(section))
    spatial_output_file_no_fit = join(scope_name, "NoFitSpatialIntensity{}.svg".format(section))
    # output_file = join(volume_dir, section.rjust(4, "0"), "Intensity.png")
    title = "Section {} - {}".format(section, whichTEM(idoc))

    # Plot spatial intensity with and without Jamie's plane fit
    PlotSpatialIntensity(True, True, idoc, log, spatial_output_file, title)
    PlotSpatialIntensity(True, False, idoc, log, spatial_output_file_no_fit, title)

    #PlotSpatialIntensity(False, idoc, log, None, title) # uncomment this to view the 3D plot interactively
    plot.PolyLine(minMaxMeanData(idoc, log), title , "Time", "Intensity", output_file, LineWidth=0)

def FitPlane(points):
    '''Fit a plane to a 3D set of points in a numpy array
    :return: A tuple of the coefficients for X,Y,Z
    '''
 
    #points = points - np.min(points,0)
    num_points = points.shape[0]
    tmp_a = points[:,0:2] #XY values
    tmp_a = np.hstack((tmp_a, np.ones((num_points,1))))
    tmp_b = points[:,-1] #Z values
    
    b = np.matrix(tmp_b).T
    A = np.matrix(tmp_a)
    
    fit = (A.T * A).I * A.T * b #find a linear fit for X + Y = Z
    errors = b - A * fit
    residual = np.linalg.norm(errors)

    return (fit, errors, residual)

def SubtractPlanarFitFromPoints(points):
    
    num_points = points.shape[0]
    (fit, error, residual) = FitPlane(points)

    defocus_solution = "%f x + %f y + %f = z" % (fit[0], fit[1], fit[2])
    print( defocus_solution )

    tmp_a = points[:,0:2] #XY values
    tmp_a = np.hstack((tmp_a, np.ones((num_points,1))))
    remapped = tmp_a * fit.flat
    adjusted_z = np.sum(remapped,1)



    #np.Array(points)
    point_copy = np.array(points)
    point_copy[:,2] = points[:,2] - adjusted_z
    return point_copy


def PlotSpatialIntensity(Flat, DoFitPlane, IDocSource, LogSource, OutputImageFile=None, title=None):

    Data = ArgToIdoc(IDocSource)
    section_log = ArgToSerialEMLog(LogSource)

    timeStamps = [tile.startTime for tile in section_log.tileData.values()]

    assert Data is not None
    
    if title is None:
        title = 'Spatial position vs Intensity'

    title += " "
    if DoFitPlane:
        title += "with planar fit"
    else:
        title += "without planar fit"
    
    x = []
    y = []
    z = []
    
    first_tile = Data.Tiles[0]
    center = np.asarray((first_tile.StagePosition)) 
    
    points = None

    min_x = None

    left_end = None

    timeStamps = [] #[tile.startTime for tile in section_log.tileData.values()]

    (f_root, f_ext) = splitext(basename(Data.Tiles[0].Image))
    first_image_number = int(f_root)
    
    for t in list(Data.Tiles):
        if not t.Mean is None:
             
            if min_x is None:
                min_x = t.PieceCoordinates[0]
            elif t.PieceCoordinates[0] < min_x:
                min_x = t.PieceCoordinates[0]
            elif t.PieceCoordinates[0] > min_x and left_end is None: #We've returned to center
                left_end = points.shape[0]
            
            (root, ext) = splitext(basename(t.Image)) 
            tile_number = int(root) - first_image_number
            timeStamps.append(section_log.tileData[tile_number].startTime)
            #x.append(t.StagePosition[0])
            #y.append(t.MeanStagePosition[1])
            #z.append(t.Mean)
            
            row = np.asarray((t.StagePosition[0],t.StagePosition[1],t.Mean))
            #row = np.swapaxes(row, 0, 1)
            if points is None:
                points = row
            else:
                points = np.vstack((points,row))

    num_points = points.shape[0]
    timeStamps = np.array(timeStamps)

    print("num timestamps {0}".format(str(timeStamps.shape)))
    print("num points {0}".format(str(points[:,2].shape)))


            
    #Adjust so capture start position is at 0,0
    points[:,0:2] = points[:,0:2] - center  
    
    #print( "errors:")
    #print( errors)
    #print( "residual:")
    #print( residual)

    #print( "solution:")

    
    left_points = points[0:left_end,:]
    right_points = points[left_end:,:]
    if DoFitPlane:
        left_points = SubtractPlanarFitFromPoints(left_points)
        right_points = SubtractPlanarFitFromPoints(right_points)

    adjusted_points = np.vstack((left_points, right_points))

    adjusted_points[:, 2] = adjusted_points[:, 2] - np.mean(adjusted_points[:, 2])
    z = adjusted_points[:, 2]
    #title = "Defocus recorded at each capture position in mosaic\nradius = defocus, color = # of tries"
     
    fig = plt.figure(dpi=150)
    ax = fig.add_subplot(111, projection='3d')
    
    triang = mtri.Triangulation( adjusted_points[:,0],  adjusted_points[:,1])
    
    zrange = np.max(np.abs((np.min(z), np.max(z))))
    #AllowedZLimits = [1.0, 2.5, 5.0, 10.0, 25.0, 50, 100, 1000, 10000, 20000, 30000, 35000, 40000, 45000, 50000, 55000, 60000, 65000]
    #zlim = NearestLimit(zrange, AllowedZLimits)
    
    #if zrange < 1.0:
    #    zrange = 1.0
    
    offset = SymmetricNormalize(vabsmax=zrange, vcenter=0.)
    
    ax.plot_trisurf(triang, z, cmap=plt.get_cmap('plasma'), shade=True, alpha=1, norm=offset) #, c=c, Title=title, XAxisLabel='X', YAxisLabel='Y', OutputFilename=OutputImageFile)
    ax.set_title(title)
    if Flat:
        ax.set_zticks([0])
    else:
        ax.set_zlabel('Z (intensity)')
    
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    ax.set_zlim(-zrange, zrange)
    #interact(local = locals())
    ax.set_xlim(np.min(adjusted_points[:,0]), np.max(adjusted_points[:,0]) )
    ax.set_ylim(np.min(adjusted_points[:,1]), np.max(adjusted_points[:,1]) )
    
    fig.subplotpars.left = 0
    fig.subplotpars.right = 1
    fig.subplotpars.bottom = 0
    fig.subplotpars.top = 1
    
    if Flat:
        ax.view_init(90, -90)
    else:
        ax.view_init(30, -90)

    if OutputImageFile is None:
        plt.show()
    else: 
        plt.ioff()
        plt.savefig(OutputImageFile, bbox_inches='tight', dpi=300)
    
    plt.close(fig) 

    print("min z: {0}".format(np.min(adjusted_points[:,2])))
    #plot.Scatter(timeStamps, adjusted_points[:,2], Title=title, XAxisLabel="Tile X", YAxisLabel="Tile Y", ZAxisLabel="Mean Intensity", OutputFilename=None)
    #plot.PolyLine([[timeStamps, points[0:2,:]]], title , "Time", "Intensity", None, LineWidth = 0) #"scatter_" + output_file, LineWidth=0)

        
    return

if __name__ == "__main__":
    volume_dir = ""
    sections = []
    if len(sys.argv) > 1:
        assert exists(sys.argv[1]), "First arg must specify the directory of a volume's raw data."
        volume_dir = sys.argv[1]
    else:
        print("First arg must specify the directory of a volume's raw data.")
        exit()
    if len(sys.argv) > 2:
        sections = parseSections(sys.argv[2])
    if len(sections) == 0:
        print("Second arg must specify one or more sections to correct.")
        exit()

    for section in sections:
        plotIntensity(volume_dir, section)

    
# Notes for making actual value correction:
#img = nornir_imageregistration.Load(".png")
#img = img + intensity_adjustment # adjustment = planar adjustment(x,y) + curve fit(time)
#nornir_imageregistration.Save(img)