import sys
from pathlib import Path

sys.path.insert(1, '../build')

from BF1942_Extraction_Readout_Scripts.meme_file import MemeFile

# Export the binary menu files (files with no extension) in src/menu to their .meme.xml form
for file in Path("../src/menu").iterdir():
    if file.is_file() and "." not in file.name:
        print(f"Exporting {file}")
        meme_file = MemeFile(file)
        meme_file.read()
        meme_file.save_to_xml()
