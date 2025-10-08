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
- **verify <token> <key>**: Decodes, analyzes claims, and verifies the signature using the provided secret key.

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

### Verifying a Token
Type:
```
/jwt verify eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c secret
```

Additional Output:
- **Verification**: "Signature is valid." or "**Warning:** Signature is invalid."

## Supported Algorithms
- HS256 (HMAC with SHA-256)
- HS384 (HMAC with SHA-384)
- HS512 (HMAC with SHA-512)
- none (Unsecured JWTs; verification checks if signature is empty)

Note: RSA algorithms are not currently supported. Work in progress.

## License

MIT License.
