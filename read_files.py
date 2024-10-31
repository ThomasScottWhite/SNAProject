import os
import glob
import json
import re


def data():
    comment_files = glob.glob("./comments/*")
    conversations_files = glob.glob("./conversations/*")
    submissions_files = glob.glob("./submissions/*")
    data_list = []
    for (
        comment,
        conversation,
        submission,
    ) in zip(comment_files, conversations_files, submissions_files):
        with open(comment, "r") as f:
            comment_data = json.load(f)
        with open(conversation, "r") as f:
            conversation = json.load(f)
        with open(submission, "r") as f:
            submission = json.load(f)

        match = re.search(r"/(\d{4})_(\d{2})\.json$", comment)
        if match:
            year_month = f"{match.group(1)}_{match.group(2)}"

        data_list.append(
            {
                "year_month": year_month,
                "comment_data": comment_data,
                "conversation_data": conversation,
                "submission_data": submission,
            }
        )
    return data_list
