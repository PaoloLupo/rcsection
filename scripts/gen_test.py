import os
from pathlib import Path

def main():
    # Define paths relative to the script location or CWD
    # Assuming script is run from root or we can find root
    # The user said "call by justfile", so likely CWD is root.
    
    root_dir = Path.cwd()
    examples_dir = root_dir / 'examples'
    tests_dir = root_dir / 'tests'
    
    if not examples_dir.exists():
        print(f"Error: examples directory not found at {examples_dir}")
        return

    # Template content based on tests/minimal/test.typ
    # We need to replace the filename in read()
    template = """#import "/src/rcsection.typ": *

#set page(height: auto, width: auto, margin: 2pt)
#set text(lang: "es")
#show: init_rcsection

#raw(
  block: true,
  lang: "rcs",
  read("../../examples/{filename}").trim("\\n"),
)
"""

    # Iterate over .rcs files
    for rcs_file in examples_dir.glob('*.rcs'):
        test_name = rcs_file.stem
        test_dir = tests_dir / test_name
        
        # Create test directory if it doesn't exist
        test_dir.mkdir(parents=True, exist_ok=True)
        
        test_file_path = test_dir / 'test.typ'
        
        # Generate content
        content = template.format(filename=rcs_file.name)
        
        # Write to file
        with open(test_file_path, 'w') as f:
            f.write(content)
            
        print(f"Generated test for {rcs_file.name} in {test_dir}")

if __name__ == '__main__':
    main()
