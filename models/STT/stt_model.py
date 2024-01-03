from transformers import Wav2Vec2ForCTC, Wav2Vec2Tokenizer
from datasets import load_dataset
import torch
import librosa

class STT_Model:

  def __init__(self):
    self.tokenizer = Wav2Vec2Tokenizer.from_pretrained("facebook/wav2vec2-base-960h")
    self.model = Wav2Vec2ForCTC.from_pretrained("facebook/wav2vec2-base-960h")
    
  

audio, rate = librosa.load("../data/miggli.wav", sr = 16000)

# tokenize
input_values = tokenizer(audio, return_tensors = "pt").input_values

# retrieve logits
logits = model(input_values).logits

# take argmax and decode
predicted_ids = torch.argmax(logits, dim = -1)
transcription = tokenizer.batch_decode(predicted_ids)[0]

print("Transcription: ", transcription)