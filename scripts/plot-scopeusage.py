import sys
from util import *
from process_sections import *
from datetime import datetime, time
import nornir_shared.plot as plot

def dayNum(dt):
    ''' Return the day number of the given datetime.

        January 1, year 1  ------> 0
        August 4, 2020 ---------> 737640
    '''
    return (dt - datetime(1, 1, 1)).days


def polyLineData(intervals):
    lines = []
    for interval in intervals:
        xAxis = [
            interval[1].hour * 60 + interval[1].minute,
            # Allow for times past midnight to be on the same y coordinate as the previous day
            (interval[2].day - interval[1].day) * (24 * 60) + interval[2].hour * 60 + interval[2].minute
        ]
        yValue = -dayNum(interval[1])
        # Stagger TEM1 and TEM2 lines:
        if interval[0] == "TEM2":
            yValue -= 0.5
        yAxis = [yValue, yValue]
        lines.append([xAxis, yAxis])
    return lines

def polyLineColors(intervals):
    # TEM1 captures will be red, TEM2 captures will be blue.
    colors = []
    for interval in intervals:
        if interval[0] == "TEM1":
            colors.append("red")
        elif interval[0] == "TEM2":
            colors.append("blue")
        else:
            raise "This script only supports 2 TEMs. Something went wrong"
    return colors

def plotCaptureIntervals(file, title, intervals):
    relevantHours = range(7, 24+7) # 24-hour interval starting at 7 AM
    xTicks = [hour*60 for hour in relevantHours]
    xTickLabels = [time(hour=hour % 24).strftime("%I %p") for hour in relevantHours]
    startTimes = [interval[1] for interval in intervals]
    yTicks = [-dayNum(dt) for dt in startTimes]
    yTickLabels = [dt.strftime("%a %b %d") for dt in startTimes]

    plot.PolyLine(
        polyLineData(intervals),
        # Start the x axis at 7 AM, and extend it into the next early morning
        xlim=(7*60, (24+7)*60),
        Title=title, OutputFilename=file,
        #XAxisLabel="Time of day", 
        #YAxisLabel="Day",
        XTicks=xTicks,
        XTickLabels=xTickLabels,
        XTickRotation=-90,
        YTicks=yTicks,
        YTickLabels=yTickLabels,
        ColorStyle=plot.ColorSelectionStyle.PER_LINE,
        Colors=polyLineColors(intervals))

def captureIntervals(section, idoc, log):
    return [whichTEM(idoc), log.StartupDateTime, log.FinishDateTime]

if __name__ == "__main__":
    usageString = "Usage: python plot-scopeusage.py [volume dir] [section range]"
    assert len(sys.argv) > 2, usageString
    assert exists(sys.argv[1]), usageString
    
    volumeDir = sys.argv[1]
    sectionsRange = sys.argv[2]

    captureIntervals = processSections(volumeDir, sectionsRange, captureIntervals)

    # Separate charts by TEM:
    #tem1CaptureIntervals = filter(lambda interval: interval[0] == "TEM1", captureIntervals)
    #tem2CaptureIntervals = filter(lambda interval: interval[0] == "TEM2", captureIntervals)

    #plotCaptureIntervals("TEM1ScopeUsage.png", "TEM1 Scope Usage", tem1CaptureIntervals)
    #plotCaptureIntervals("TEM2ScopeUsage.png", "TEM2 Scope Usage", tem2CaptureIntervals)
    
    # Both tems on the same chart:
    plotCaptureIntervals("BothScopeUsage.png", "Both Scopes Usage", captureIntervals)