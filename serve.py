import json
from flask import Flask, jsonify
from flask_cors import CORS

app = Flask(__name__)
CORS(app)


@app.route('/')
def index():
    return 'Development server running'


@app.route('/criterias')
def criterias():
    print("get criterias")

    with open('criterias.json', 'r') as file:
        data = json.load(file)
        return data

@app.route('/params')
def params():
    print("get params")

    with open('data.json', 'r') as file:
        content = file.read()
        return content

if __name__ == '__main__':
    app.run(port=1984)
