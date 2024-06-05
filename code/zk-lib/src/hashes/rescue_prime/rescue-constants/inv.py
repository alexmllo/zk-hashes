hex_value = "b76ddbb6489224496d27b66d4a4651ff6f13b3287113ab83435afa5a91feae31"

# Convert hexadecimal string to bytes
binary_data = bytes.fromhex(hex_value)

# Write the binary data to a .bin file
with open("output_file.bin", "wb") as file:
    file.write(binary_data)

print("Binary file 'output_file.bin' created successfully.")