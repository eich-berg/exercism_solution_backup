def proverb(*input_data, qualifier=None):
    if not input_data: return []
    output = ""  
    lines = [
        f"For want of a {input_data[i]} the {input_data[i+1]} was lost."
        for i in range(len(input_data) - 1)
    ]
    final_item = f"{qualifier} {input_data[0]}" if qualifier else input_data[0]
    lines.append(f"And all for the want of a {final_item}.")
    return lines


