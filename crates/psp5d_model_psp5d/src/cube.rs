use psp5d_core::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeConfig {
    pub n: usize,
    pub m: u32,
}

pub fn validate_coord(coord: &[u32], cfg: &CubeConfig) -> Result<(), CoreError> {
    if coord.len() != cfg.n {
        return Err(CoreError::InvariantViolation(
            "coord arity mismatch".to_string(),
        ));
    }
    if coord.iter().any(|v| *v == 0 || *v > cfg.m) {
        return Err(CoreError::InvariantViolation(
            "coord outside 1..=m domain".to_string(),
        ));
    }
    Ok(())
}

pub fn neighbors_local_moves(coord: &[u32], cfg: &CubeConfig) -> Result<Vec<Vec<u32>>, CoreError> {
    validate_coord(coord, cfg)?;
    let mut out = Vec::new();
    for i in 0..cfg.n {
        if coord[i] > 1 {
            let mut c = coord.to_vec();
            c[i] -= 1;
            out.push(c);
        }
        if coord[i] < cfg.m {
            let mut c = coord.to_vec();
            c[i] += 1;
            out.push(c);
        }
    }
    Ok(out)
}

pub fn enforce_local_move(a: &[u32], b: &[u32]) -> Result<(), CoreError> {
    if a.len() != b.len() {
        return Err(CoreError::InvariantViolation(
            "local move arity mismatch".to_string(),
        ));
    }
    let diff_axes = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count();
    if diff_axes != 1 {
        return Err(CoreError::InvariantViolation(
            "local move must differ in exactly one axis".to_string(),
        ));
    }
    Ok(())
}
