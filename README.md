# hardware-sampler

A tool for detecting and sampling hardware attestation evidence from various Trusted Execution Environment (TEE) types.

## Overview

`hardware-sampler` automatically detects available attestable hardware devices and collects attestation evidence from them. It supports multiple TEE types and formats the evidence as structured claims in the same format as [Trustee](https://github.com/confidential-containers/trustee).

## Supported TEE Types

- **Azure SNP vTPM** (`AzSnpVtpm`) - Azure Confidential Computing with SEV-SNP and vTPM
- **Azure TDX vTPM** (`AzTdxVtpm`) - Azure Confidential Computing with Intel TDX and vTPM (optional feature)
- **SEV-SNP** (`Snp`) - AMD SEV-SNP attestation
- **TPM** - Trusted Platform Module devices

## Features

- Automatic detection of available TEE types
- Evidence collection from multiple attestable devices
- Parsing and formatting of attestation claims

## Usage

just run `sudo podman run --privileged quay.io/rkaufman/hardware-sampler`


## Output

The tool outputs structured JSON claims for each detected TEE type.

