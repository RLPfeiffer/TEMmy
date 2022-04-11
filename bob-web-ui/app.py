from flask import Flask
from . import volume
from os.path import dirname
from os.path import normpath
app = Flask(__name__)

# TODO make a checkAll function and route
@app.route('/<str:volume_name>/<int:start>/<int:end>')
def volume_manager(volume_name, start, end):
    return volume.checkFrom(volume_name, start, end)

@app.route('/build/<volume_name>/<section>')
def build(volume_name, section):
    return volume.build(volume_name, section)

@app.route('/rebuild/<volume_name>/<section>')
def rebuild(volume_name,section):
    return volume.rebuild(volume_name, section)

@app.route('/fixmosaic/<volume_name>/<section>')
def fixmosaic(volume_name, section):
    return volume.fixmosaic(volume_name, section)

@app.route('/merge/<volume_name>/<section>')
def merge(volume_name, section):
    return volume.merge(volume_name, section)

@app.route('/file/<path:p>')
def file(p):
    with open(p, 'r') as f:
        dir = dirname(p)
        dir = dir.replace('W:', 'file://///OpR-Marc-Syn2/Data')
        content = f.read()
        # this is a godawful hack
        content = content.replace('href="', f'href="{dir}/')
        content = content.replace('src="', f'src="{dir}/')
        content = content.replace("\\", '/')
        return content

if __name__ == "__main__":
    app.run(debug=True)