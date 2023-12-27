# exifimg

## Summary

This utility is a Rust command-line application that reads and optionally strips EXIF data from images. It uses the exif crate for handling EXIF metadata and the image crate for basic image processing.

## Arguments

- `file_path`: This is a required argument. You must provide the path to the image file you want to work with.
- `--recursive` or `-r`: This is an optional argument. If provided, the utility will operate on files in the given directory and all subdirectories.
- `--strip` or `-s`: This is an optional argument. If provided, the utility will remove the EXIF data from the image file.
