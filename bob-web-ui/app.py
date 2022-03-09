from flask import Flask
from . import rc3
from os.path import dirname
from os.path import normpath
app = Flask(__name__)

@app.route('/rc3/<int:start>/<int:end>')
def rc3_manager(start, end):
    return rc3.checkFrom(start, end)

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