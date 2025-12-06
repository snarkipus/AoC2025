# Project Architecture Rules (Non-Obvious Only)
- Days are independent crates; use workspace deps for shared crates like nom, rayon
- Process functions standardized to miette::Result<String>