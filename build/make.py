import os
import sys
import shutil
from BF1942_Extraction_Readout_Scripts.refractor_flat_archive import RefractorFlatArchive
from BF1942_Extraction_Readout_Scripts.lexicon import LexiconFile

mod_name = "GCMOD"

src_folder_path = "../src"

levels_folder_path = f"{src_folder_path}/bf1942/levels"
game_rfa_path = f"{src_folder_path}/bf1942/game.rfa"

level_paths = [f"{levels_folder_path}/{name}" for name in os.listdir(levels_folder_path)]
mod_archive_paths = [f"{src_folder_path}/{name}" for name in os.listdir(src_folder_path) if not name in ["bf1942", "Mods"]]
if os.path.isfile(game_rfa_path):
    mod_archive_paths.append(f"{src_folder_path}/bf1942/game")

archive_paths = mod_archive_paths + level_paths

if os.path.exists(mod_name):
    shutil.rmtree(mod_name)

for archive_path in archive_paths:
    relative_path = os.path.relpath(archive_path, src_folder_path)
    rfa_file_path = f"{mod_name}/Archives/{relative_path}.rfa"
    
    rfa = RefractorFlatArchive(rfa_file_path, read=False)
    rfa.add_directory(archive_path, src_folder_path)
    
    folder_path = os.path.split(rfa_file_path)[0]

    if not os.path.exists(folder_path):
        os.makedirs(folder_path)
        
    rfa.write(rfa_file_path)

shutil.copytree(f"{src_folder_path}/Mods/{mod_name}", mod_name, dirs_exist_ok=True)

copied_logs_folder_path = f"{mod_name}/logs"
if os.path.exists(copied_logs_folder_path):
    shutil.rmtree(copied_logs_folder_path)

# convert lexiconAll.xml to lexiconAll.dat
lexicon_file_path = f"{mod_name}/lexiconAll.xml"
if os.path.isfile(lexicon_file_path):
    lex = LexiconFile(f"{mod_name}/lexiconAll.dat")
    lex.load_from_xml()
    lex.write()
    os.remove(lexicon_file_path)
