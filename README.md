# rbc Really Bad Cipher
## A Caesar Cipher in Rust
A caesar cipher is the simplest cipher known to man. It's simply taking every letter in your message, and moving it a constant value to the right or left. 
```
ABDCEFG 
// Shifted 1 unit left
BDCEFGH
```

## Usage:
Once compiled and installed through cargo, simply run in the command line like so.
### A simple message
```
echo "This is a terrible cipher" | rbc 5
Ymnx nx f yjwwngqj hnumjw!
```
### A file
```
cat README.md | rbc 5
# wgh Wjfqqd Gfi Hnumjw
```
### Saving the ciphered file
```
cat README.md | rbc 5 >> README.cipher.md
# wgh Wjfqqd Gfi Hnumjw
```
