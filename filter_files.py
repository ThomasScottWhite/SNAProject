import os
import glob
import json
import re
import pickle
import gc

supporting_israel = [
    "Jewish", 
    "Judaism", 
    "IsraelUnderAttack", 
    "IsraelPalestine", 
    "IsraelICYMI", 
    "IsraelWar", 
    "Israel", 
    "IsraelVsHamas"
]

opposing_israel = [
    "Palestine", 
    "IsraelPalestine", 
    "AskMiddleEast", 
    "IsraelHamasWar", 
    "islam", 
    "israelexposed", 
    "exmuslim", 
    "IsraelCrimes", 
    "PalestinianViolence", 
    "AntiSemitismInRedditIsraelWarVideoReport", 
    "MuslimLounge", 
    "Muslim", 
    "Gaza", 
    "MuslimCorner", 
    "PalestinianvsIsrael"
]

comment_files = glob.glob("./comments/*")
conversations_files = glob.glob("./conversations/*")
submissions_files = glob.glob("./submissions/*")

os.makedirs("./filtered_comments", exist_ok=True)
os.makedirs("./filtered_conversations", exist_ok=True)
os.makedirs("./filtered_submissions", exist_ok=True)

print("Filtering files")

for comment_file, conversation_file, submission_file in zip(comment_files, conversations_files, submissions_files):

    with open(comment_file, 'r') as f:
        comment_data = json.load(f)
        filtered_comment_data = [comment for comment in comment_data if comment['subreddit'] in supporting_israel or comment['subreddit'] in opposing_israel]
        filtered_comment_file = f"./filtered_comments/{os.path.basename(comment_file)}"
        with open(filtered_comment_file, 'w') as f:
            json.dump(filtered_comment_data, f)

        del comment_data
        del filtered_comment_data
        gc.collect()
        # Process conversations

    with open(conversation_file, 'r') as f:
        conversation_data = json.load(f)
        filtered_conversation_data = [
            conversation for conversation in conversation_data
            if conversation['subreddit'] in supporting_israel or conversation['subreddit'] in opposing_israel
        ]
        filtered_conversation_file = f"./filtered_conversations/{os.path.basename(conversation_file)}"
        with open(filtered_conversation_file, 'w') as f:
            json.dump(filtered_conversation_data, f)
        del conversation_data
        del filtered_conversation_data
        gc.collect()

    # Process submissions
    with open(submission_file, 'r') as f:
        submission_data = json.load(f)
        filtered_submission_data = [
            submission for submission in submission_data
            if submission['subreddit'] in supporting_israel or submission['subreddit'] in opposing_israel
        ]
        filtered_submission_file = f"./filtered_submissions/{os.path.basename(submission_file)}"
        with open(filtered_submission_file, 'w') as f:
            json.dump(filtered_submission_data, f)
        del submission_data
        del filtered_submission_data
        gc.collect()

print("Done filtering files")