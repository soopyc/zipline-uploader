# zipline-uploader

convenient cli file uploader to zipline hosts

## dependencies

- Rust (1.86+)
- libmagic (file)
- zipline (``~v4``, runtime)

## configuration options

```toml
remote_url = "https://zipline.host.invalid" # the zipline host that you would like to use.
token = "AAAAAAAAAAAAAA==.PASTE/THE/TOKEN/HERE/FROM+//+ZIPLINE" # the token.

save_original_name = true # whether to save the original filename.
filename_format = "unset" # a list of: random|date|uuid|name|gfycat

# list of headers to merge with the internal list, replacing ones that already exist.
# for zipline options, see https://zipline.diced.sh/docs/guides/upload-options#headers
[[extra_headers]]
name = "x-zipline-filename"
value = "somerandomvalue"
```
