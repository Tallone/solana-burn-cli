# Windows Build Guide

This document explains how to build the Solana Burn CLI on Windows and the solutions implemented for common build issues.

## OpenSSL Dependency Issue

### Problem
The Solana client libraries depend on OpenSSL, which can cause build failures on Windows with the error:
```
Could not find directory of OpenSSL installation, and this `-sys` crate cannot
proceed without this knowledge.
```

### Solution
We use vcpkg to install OpenSSL for Windows builds in our CI/CD pipeline:

```yaml
- name: Install OpenSSL (Windows)
  if: matrix.os == 'windows-latest'
  run: |
    echo "VCPKG_ROOT=$env:VCPKG_INSTALLATION_ROOT" | Out-File -FilePath $env:GITHUB_ENV -Append
    vcpkg install openssl:x64-windows-static-md
```

## Local Development on Windows

### Prerequisites
1. **Rust**: Install from [rustup.rs](https://rustup.rs/)
2. **Visual Studio Build Tools**: Install Visual Studio 2019/2022 with C++ build tools
3. **vcpkg** (optional): For OpenSSL installation

### Option 1: Using vcpkg (Recommended)

1. Install vcpkg:
   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   .\vcpkg integrate install
   ```

2. Install OpenSSL:
   ```powershell
   .\vcpkg install openssl:x64-windows-static-md
   ```

3. Set environment variables:
   ```powershell
   $env:VCPKG_ROOT = "C:\path\to\vcpkg"
   ```

4. Build the project:
   ```powershell
   cargo build --release
   ```

### Option 2: Using Chocolatey

1. Install Chocolatey from [chocolatey.org](https://chocolatey.org/)

2. Install OpenSSL:
   ```powershell
   choco install openssl
   ```

3. Set environment variables:
   ```powershell
   $env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
   $env:OPENSSL_STATIC = "1"
   ```

4. Build the project:
   ```powershell
   cargo build --release
   ```

### Option 3: Using Pre-built OpenSSL

1. Download pre-built OpenSSL from [Shining Light Productions](https://slproweb.com/products/Win32OpenSSL.html)

2. Install to default location (usually `C:\Program Files\OpenSSL-Win64`)

3. Set environment variables:
   ```powershell
   $env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
   $env:OPENSSL_STATIC = "1"
   ```

4. Build the project:
   ```powershell
   cargo build --release
   ```

## Troubleshooting

### Common Issues

#### 1. "link.exe not found"
**Solution**: Install Visual Studio Build Tools with C++ support.

#### 2. "OpenSSL not found"
**Solution**: Ensure OpenSSL is installed and environment variables are set correctly.

#### 3. "Permission denied" errors
**Solution**: Run PowerShell as Administrator when installing dependencies.

#### 4. vcpkg integration issues
**Solution**: Run `vcpkg integrate install` as Administrator.

### Environment Variables

Make sure these environment variables are set correctly:

```powershell
# For vcpkg
$env:VCPKG_ROOT = "C:\path\to\vcpkg"

# For OpenSSL
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"  # or vcpkg path
$env:OPENSSL_STATIC = "1"

# For linking
$env:OPENSSL_LIB_DIR = "$env:OPENSSL_DIR\lib"
$env:OPENSSL_INCLUDE_DIR = "$env:OPENSSL_DIR\include"
```

### Verification

To verify your setup works:

1. Check Rust installation:
   ```powershell
   rustc --version
   cargo --version
   ```

2. Check OpenSSL installation:
   ```powershell
   # For vcpkg
   vcpkg list | findstr openssl
   
   # For Chocolatey/manual install
   Test-Path "C:\Program Files\OpenSSL-Win64\bin\openssl.exe"
   ```

3. Test build:
   ```powershell
   cargo check
   ```

## CI/CD Configuration

Our GitHub Actions workflow automatically handles Windows builds:

- Uses `windows-latest` runner
- Installs OpenSSL via vcpkg
- Sets appropriate environment variables
- Builds static binaries for distribution

The workflow is configured to:
1. Install vcpkg and OpenSSL
2. Cache dependencies for faster builds
3. Build release binaries
4. Create distribution packages

## Performance Notes

- vcpkg installation takes 5-10 minutes in CI
- We cache the vcpkg installation to speed up subsequent builds
- Static linking is used to create standalone executables

## Alternative Solutions

If you encounter persistent issues, consider:

1. **Using WSL2**: Build in a Linux environment on Windows
2. **Docker**: Use a containerized build environment
3. **Cross-compilation**: Build from Linux targeting Windows

## Support

If you encounter Windows-specific build issues:

1. Check the GitHub Actions logs for the latest working configuration
2. Ensure your local environment matches the CI setup
3. Try the troubleshooting steps above
4. Open an issue with your specific error message and environment details
