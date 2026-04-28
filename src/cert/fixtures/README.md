# Certificate Test Fixtures

These files are public, generated-only test fixtures for DevTools certificate parser unit tests.

- `sample.pem` — self-signed RSA X.509 certificate in PEM format.
- `sample.der` — the same certificate converted to DER format.
- `chain.pem` — two self-signed public test certificates concatenated to exercise multi-PEM parsing.
- `sample.p12` — PKCS#12 container with the public sample certificate only.

The PKCS#12 password is `test-password`. It is documented test data, not a secret. Temporary private keys used to generate these fixtures were deleted after export and are not checked in.
