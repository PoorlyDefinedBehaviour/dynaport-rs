use std::net::TcpListener;

use rand::seq::SliceRandom;
use thiserror::Error;

/// The Registered Ports (1024-49151) – which can be used by applications, specific services, and users.
const LOWEST_REGISTERED_PORT: usize = 1024;
const HIGHEST_REGISTERED_PORT: usize = 49151;

/// The Dynamic and/or Private Ports (49152-65535)
const LOWEST_DYNAMIC_PORT: usize = 49152;
const HIGHEST_DYNAMIC_PORT: usize = 65535;

#[derive(Debug, PartialEq, Error)]
pub enum DynaportError {
  #[error("wanted {wanted:?} ports but there are only {got:?} available")]
  NotEnoughPorts { wanted: usize, got: usize },
}

fn is_available(port: usize) -> bool {
  TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

/// Returns a registered port that is not being used.
///
/// The port is chosen at random.
pub fn random_registered_port() -> Option<usize> {
  let mut ports: Vec<usize> = (LOWEST_REGISTERED_PORT..=HIGHEST_REGISTERED_PORT).collect();

  ports.shuffle(&mut rand::thread_rng());

  for port in ports {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the lowest registered port that is not being used.
pub fn lowest_registered_port() -> Option<usize> {
  for port in LOWEST_REGISTERED_PORT..=HIGHEST_REGISTERED_PORT {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the n lowest registered ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
pub fn lowest_n_registered_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in LOWEST_REGISTERED_PORT..=HIGHEST_REGISTERED_PORT {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if let Ok(_) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
      ports.push(port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

/// Returns the highest registered port that is not being used.
pub fn highest_registered_port() -> Option<usize> {
  for port in (LOWEST_REGISTERED_PORT..=HIGHEST_REGISTERED_PORT).rev() {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the n highest registered ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
pub fn highest_n_registered_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in (LOWEST_REGISTERED_PORT..=HIGHEST_REGISTERED_PORT).rev() {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if let Ok(_) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
      ports.push(port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

/// Returns the lowest dynamic port that is not being used.
pub fn lowest_dynamic_port() -> Option<usize> {
  for port in LOWEST_DYNAMIC_PORT..=HIGHEST_DYNAMIC_PORT {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the n lowest dynamic ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
pub fn lowest_n_dynamic_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in LOWEST_DYNAMIC_PORT..=HIGHEST_DYNAMIC_PORT {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if let Ok(_) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
      ports.push(port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

/// Returns the highest dynamic port that is not being used.
pub fn highest_dynamic_port() -> Option<usize> {
  for port in (LOWEST_DYNAMIC_PORT..=HIGHEST_DYNAMIC_PORT).rev() {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the n highest dynamic ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
pub fn highest_n_dynamic_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in (LOWEST_DYNAMIC_PORT..=HIGHEST_DYNAMIC_PORT).rev() {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if let Ok(_) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
      ports.push(port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_random_registered_port() {
    assert!(matches!(random_registered_port(), Some(_)));
  }

  #[test]
  fn test_lowest_registered_port() {
    let mut listeners = vec![];

    for expected_port in [1024, 1025, 1026] {
      assert_eq!(Some(expected_port), lowest_registered_port());

      listeners.push(TcpListener::bind(format!("127.0.1:{}", expected_port)).unwrap());
    }
  }

  #[test]
  fn test_lowest_n_registered_ports() {
    assert_eq!(Ok(vec![1024, 1025, 1026]), lowest_n_registered_ports(3));
  }

  #[test]
  fn test_highest_registered_port() {
    let mut listeners = vec![];

    for expected_port in [49151, 49150, 49149] {
      assert_eq!(Some(expected_port), highest_registered_port());

      listeners.push(TcpListener::bind(format!("127.0.1:{}", expected_port)).unwrap());
    }
  }

  #[test]
  fn test_highest_n_registered_ports() {
    assert_eq!(Ok(vec![49151, 49150, 49149]), highest_n_registered_ports(3));
  }

  #[test]
  fn test_lowest_dynamic_port() {
    let mut listeners = vec![];

    for expected_port in [49152, 49153, 49154] {
      assert_eq!(Some(expected_port), lowest_dynamic_port());

      listeners.push(TcpListener::bind(format!("127.0.1:{}", expected_port)).unwrap());
    }
  }

  #[test]
  fn test_lowest_n_dynamic_ports() {
    assert_eq!(Ok(vec![49152, 49153, 49154]), lowest_n_dynamic_ports(3));
  }

  #[test]
  fn test_highest_dynamic_port() {
    let mut listeners = vec![];

    for expected_port in [65535, 65534, 65533] {
      assert_eq!(Some(expected_port), highest_dynamic_port());

      listeners.push(TcpListener::bind(format!("127.0.1:{}", expected_port)).unwrap());
    }
  }

  #[test]
  fn test_highest_n_dynamic_ports() {
    assert_eq!(Ok(vec![65535, 65534, 65533]), highest_n_dynamic_ports(3));
  }
}
