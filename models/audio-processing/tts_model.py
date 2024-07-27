import os
from TTS.api import TTS

def use_tts(message_to_transcribe: str):
    # Initialize TTS with a pretrained model
    tts = TTS(model_name="tts_models/en/ljspeech/tacotron2-DDC", progress_bar=False, gpu=True)
    
    # Define the output directory and file path
    output_dir = os.path.join(os.path.dirname(__file__), './../data/')
    output_file = os.path.join(output_dir, 'output.mp3')
    
    # Create the directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)
    
    # Synthesize the speech and save to file
    tts.tts_to_file(text=message_to_transcribe, file_path=output_file)
