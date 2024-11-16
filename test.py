import json
import ijson
import glob
attack_keywards = ["attack", "hospital", "bomb", "kill", "injure", "violence", "war", "conflict", "fight", "combat", "battle", "assault", "strike", "clash", "offensive", "onslaught", "bombard", "besiege", "invade", "raid", "beset", "pound", "blitz", "shell", "strafe", "blow up", "destroy", "demolish", "flatten", "level", "raze", "wreck", "ruin", "annihilate", "exterminate", "eradicate", "eliminate", "extinguish", "obliterate", "decimate", "massacre", "butcher", "slaughter"]
opposing_keywords = ["oppose", "negative", "disagree"]

lists = {
    "attacks" : attack_keywards
}

comment_files = glob.glob("./filtered_comments/*")
conversations_files = glob.glob("./filtered_conversations/*")
submissions_files = glob.glob("./filtered_submissions/*")
