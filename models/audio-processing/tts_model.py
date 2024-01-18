from flask import Flask, request, jsonify
from tts_model import use_tts

app = Flask(__name__)

@app.route("/")
def hello_world():
    return "<p>MilkyTeadrop Audio Processing Api</p>"

@app.route('/api/audio/tts', methods=['POST'])
def api_tts():
    if request.method == 'POST':
        json_data = request.json
        message_to_transcribe = json_data.get('message')

        if not message_to_transcribe:
            return handle_bad_request()

        print("Message to transcribe is: ", message_to_transcribe)
        base64 = use_tts(message_to_transcribe=message_to_transcribe)

        return jsonify({"status": "ok", "base64": base64})
    else:
        return handle_bad_request()

def handle_bad_request():
    return jsonify({
        "status": "error",
        "message": "'message' is missing in request"
    }), 400

if __name__ == '__main__':
    app.run()
