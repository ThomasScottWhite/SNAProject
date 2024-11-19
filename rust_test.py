import rust_json_module
import glob

comment_files = glob.glob("./data/comments/*")
conversations_files = glob.glob("./data/conversations/*")
submissions_files = glob.glob("./data/submissions/*")


# List of JSON file paths
file_paths = comment_files + conversations_files + submissions_files

# Call the Rust function
topics = rust_json_module.process_files(file_paths)

# Access the results
for topic in topics:
    print(f"Topic: {topic.name}")
    print(f"Total References: {topic.total_references}")
    print("Supporting References per Date:")
    for date, count in topic.support_references_per_date.items():
        print(f"  {date}: {count}")
    print("Opposing References per Date:")
    for date, count in topic.oppose_references_per_date.items():
        print(f"  {date}: {count}")
    print("Neutral References per Date:")
    for date, count in topic.neutral_references_per_date.items():
        print(f"  {date}: {count}")