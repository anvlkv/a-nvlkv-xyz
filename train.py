# from accelerate.utils import write_basic_config

# write_basic_config()

audio_dataset = Dataset.from_dict({"audio": ["path/to/audio_1", ..., "path/to/audio_n"]}).cast_column("audio", Audio())