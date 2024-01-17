from flask import Flask, request
from tts_model import use_tts

app = Flask(__name__)

@app.route("/")
def hello_world():
    return "<p>MilkyTeadrop Audio Processing Api</p>"

###
# POST /api/audio/tts
# Example request:
# {
#   "message": "Hello my name is Aki, nice to meet you!" 
# }

@app.route('/api/audio/tts', methods=['POST'])
def api_tts():
    if request.method == 'POST':
        message_to_transcribe = request.form['message']
        if message_to_transcribe == "": 
            return handle_bad_request()

        print("Message to transcribe is: ", message_to_transcribe)
        use_tts(message_to_transcribe=message_to_transcribe)
        return 'NOT IMPLEMENTED'
    else: 
        return handle_bad_request()



def handle_bad_request():
    return """
    {
        "status": "error"
        "message": "'message' is missing in request"
    }""", 400



if __name__ == '__main__':
    app.run()
