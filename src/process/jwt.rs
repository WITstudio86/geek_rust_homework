use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::{Claims, JwtSign};

impl JwtSign {
    pub fn new() -> Self {
        Self {
            key: b"secret".to_vec(),
        }
    }

    pub fn sign(&self, claims: Claims) -> anyhow::Result<Vec<u8>> {
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.key),
        )?;
        Ok(token.as_bytes().to_vec())
    }

    pub fn verify(&self, token: Vec<u8>) -> anyhow::Result<(Header, Claims)> {
        let token = String::from_utf8(token)?;
        // 创建一个 Validation 实例，用于定义验证 JWT 时的规则
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["me", "device1"]);
        validation.set_required_spec_claims(&["exp", "sub", "aud"]);

        let token_data =
            decode::<Claims>(&token, &DecodingKey::from_secret(&self.key), &validation)?;

        Ok((token_data.header.clone(), token_data.claims))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        let jwt = JwtSign::new();
        let claims = Claims {
            sub: "acme".to_string(),
            aud: "device1".to_string(),
            exp: 1726409770503,
        };
        let token = jwt.sign(claims.clone()).unwrap();
        let (_, readded_claims) = jwt.verify(token).unwrap();
        assert_eq!(claims, readded_claims);
    }
}
