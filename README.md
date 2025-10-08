# zed-jwt

A Zed extension for decoding and validating JWT tokens using slash commands.

## Features

- Decode JWT tokens to view header, payload, signature, and analyzed claims (e.g., issuer, subject, expiration with warnings for expired or not-yet-valid tokens).
- Verify JWT signatures using supported algorithms.
- Auto-completions for subcommands (`decode` and `verify`) when typing `/jwt`.
- Outputs formatted sections in the Assistant panel for easy navigation.
- Analyzes standard claims like `iss`, `sub`, `aud`, `jti`, `iat`, `nbf`, and `exp`, including relative time descriptions (e.g., "2 hours ago").

## Usage

This extension provides the `/jwt` slash command for use in Zed's Assistant panel.

### Invoking the Slash Command
1. In a "You" message block in the Assistant panel, type `/` to list available slash commands.
2. Select `/jwt` (description: "JWT tools: Use 'decode <token>' or 'verify <token> <key>'").
3. After `/jwt `, type a subcommand (`decode` or `verify`); auto-completions will suggest them.
4. Provide the required arguments and press ⌘ + Enter (macOS) or Ctrl + Enter (other platforms) to submit.

The output will appear in the panel with navigable sections (e.g., Claims Analysis, Header, Payload, Signature, and Verification if applicable).

### Subcommands
- **decode <token>**: Decodes the JWT and analyzes claims. No key required.
- **verify <token> <key>**: Decodes, analyzes claims, and verifies the signature using the provided key (secret for HMAC algorithms, PEM-encoded public key for RSA algorithms).

## Examples

### Decoding a Token
Type in the Assistant panel:
```
/jwt decode eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

Example Output:
- **Claims Analysis**: Lists issuer, subject, etc., with warnings like "Warning: Token expired".
- **Header**: JSON like `{"alg": "HS256", "typ": "JWT"}`.
- **Payload**: JSON like `{"sub": "1234567890", "name": "John Doe", "iat": 1516239022}`.
- **Signature**: Base64url-encoded string.

### Verifying a Token with HMAC
Type:
```
/jwt verify eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c your-256-bit-secret
```

Additional Output:
- **Verification**: "Signature is valid." or "**Warning:** Signature is invalid."

### Verifying a Token with RSA
For RSA algorithms, provide a PEM-encoded RSA public key as the <key> argument. Example public key format:
```
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwHB3CVA4JFW3lVQbxaSH
3Kd9gYb3M4WkNzn77z0D2B1K4E4jYdY0+5c1aYgD2b3yZ9v0bW6QdY0+5c1aYgD
(remainder of the key...)
-----END PUBLIC KEY-----
```

Type:
```
/jwt verify eyJraWQiOiIxZTlnZGs3IiwiYWxnIjoiUlMyNTYifQ.ewogImlzcyI6ICJodHRwOi8vc2VydmVyLmV4YW1wbGUuY29tIiwKICJzdWIiOiAiMjQ4Mjg5NzYxMDAxIiwKICJhdWQiOiAiczZCaGRSa3F0MyIsCiAibm9uY2UiOiAibi0wUzZfV3pBMk1qIiwKICJleHAiOiAxMzExMjgxOTcwLAogImlhdCI6IDEzMTEyODA5NzAsCiAiY19oYXNoIjogIkxEa3RLZG9RYWszUGswY25YeENsdEEiCn0.XW6uhdrkBgcGx6zVIrCiROpWURs-4goO1sKA4m9jhJIImiGg5muPUcNegx6sSv43c5DSn37sxCRrDZZm4ZPBKKgtYASMcE20SDgvYJdJS0cyuFw7Ijp_7WnIjcrl6B5cmoM6ylCvsLMwkoQAxVublMwH10oAxjzD6NEFsu9nipkszWhsPePf_rM4eMpkmCbTzume-fzZIi5VjdWGGEmzTg32h3jiex-r5WTHbj-u5HL7u_KP3rmbdYNzlzd1xWRYTUs4E8nOTgzAUwvwXkIQhOh5TPcSMBYy6X3E7-_gr9Ue6n4ND7hTFhtjYs3cjNKIA08qm5cpVYFMFMG6PkhzLQ "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwHB3CVA4JFW3lVQbxaSH\n3Kd9gYb3M4WkNzn77z0D2B1K4E4jYdY0+5c1aYgD2b3yZ9v0bW6QdY0+5c1aYgD\n... (your public key here)\n-----END PUBLIC KEY-----"
```

## Supported Algorithms
- HS256 (HMAC with SHA-256)
- HS384 (HMAC with SHA-384)
- HS512 (HMAC with SHA-512)
- RS256 (RSA with SHA-256)
- RS384 (RSA with SHA-384)
- RS512 (RSA with SHA-512)
- none (Unsecured JWTs; verification checks if signature is empty)

## License

MIT License.
