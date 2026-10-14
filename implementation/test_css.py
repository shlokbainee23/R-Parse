import urllib.request
import re

html = urllib.request.urlopen('http://127.0.0.1:8080/').read().decode('utf-8')
m = re.search(r'<style>(.*?)</style>', html, re.DOTALL)
css = m.group(1)

# Check for unclosed comments
comment_opens = [m.start() for m in re.finditer(r'/\*', css)]
comment_closes = [m.start() for m in re.finditer(r'\*/', css)]
print(f"Comment /* count: {len(comment_opens)}, */ count: {len(comment_closes)}")
if len(comment_opens) != len(comment_closes):
    print("MISMATCHED COMMENTS!")

# Strip comments properly
clean_css = re.sub(r'/\*.*?\*/', '', css, flags=re.DOTALL)

# Check for remaining unclosed comments
if '/*' in clean_css:
    print("UNCLOSED COMMENT DETECTED!")
    idx = clean_css.find('/*')
    print("Near:", clean_css[idx:idx+200])

# Check for mismatched parentheses ()
paren_depth = 0
for i, line in enumerate(clean_css.splitlines(), 1):
    for ch in line:
        if ch == '(': paren_depth += 1
        elif ch == ')': paren_depth -= 1
if paren_depth != 0:
    print(f"MISMATCHED PARENTHESES: depth = {paren_depth}")

# Check for mismatched brackets []
bracket_depth = 0
for i, line in enumerate(clean_css.splitlines(), 1):
    for ch in line:
        if ch == '[': bracket_depth += 1
        elif ch == ']': bracket_depth -= 1
if bracket_depth != 0:
    print(f"MISMATCHED BRACKETS: depth = {bracket_depth}")

# Check for mismatched braces {}
brace_depth = 0
for i, line in enumerate(clean_css.splitlines(), 1):
    for ch in line:
        if ch == '{': brace_depth += 1
        elif ch == '}': brace_depth -= 1
if brace_depth != 0:
    print(f"MISMATCHED BRACES: depth = {brace_depth}")

print("Brace/paren/bracket check complete.")
