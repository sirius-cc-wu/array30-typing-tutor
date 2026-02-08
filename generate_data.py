import sys

def generate_rust_data(cin_path, output_path):
    mappings = {}
    
    with open(cin_path, 'r', encoding='utf-8') as f:
        in_chardef = False
        for line in f:
            line = line.strip()
            if not line:
                continue
            if line == '%chardef begin':
                in_chardef = True
                continue
            if line == '%chardef end':
                in_chardef = False
                continue
            
            if in_chardef:
                # Format: code char
                # Some lines might be comments or special
                if line.startswith('#'):
                    continue
                
                parts = line.split()
                if len(parts) >= 2:
                    code = parts[0]
                    char = parts[1]
                    
                    # We only care about single characters for now
                    if len(char) != 1:
                        continue
                        
                    if char not in mappings:
                        mappings[char] = []
                    mappings[char].append(code)

    # Sort by char code point
    sorted_chars = sorted(mappings.keys())
    
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("// Auto-generated Array30 data\n\n")
        f.write("pub const ARRAY30_ENTRIES: &[(char, &str)] = &[\n")
        
        for char in sorted_chars:
            codes = sorted(mappings[char], key=len) # Shortest code first
            codes_str = ",".join(codes)
            f.write(f"    ('{char}', \"{codes_str}\"),\n")
            
        f.write("];\n\n")
        
        f.write("""pub fn get_array30_code(c: char) -> Option<&'static str> {
    ARRAY30_ENTRIES.binary_search_by_key(&c, |&(k, _)| k)
        .ok()
        .map(|i| ARRAY30_ENTRIES[i].1)
}
""")

if __name__ == '__main__':
    generate_rust_data('array30.cin', 'src/array30_data.rs')
