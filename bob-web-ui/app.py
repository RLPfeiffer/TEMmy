from flask import Flask
from . import rc3
app = Flask(__name__)

@app.route('/')
def hello_geek():
    return rc3.checkFrom(1, 2000)

if __name__ == "__main__":
    app.run(debug=True)