/// Contains functions to get ports that are not being used.
///
/// Supports registered and dynamic ports.
use std::net::TcpListener;

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use thiserror::Error;

lazy_static! {
/// The Registered Ports (1024-49151) – which can be used by applications, specific services, and users.
static ref REGISTERED_PORTS_RANGE: Vec<usize> = (1024..=49151).collect();

/// The Dynamic and/or Private Ports (49152-65535)
static ref DYNAMIC_PORTS_RANGE: Vec<usize> = (49152..=65535).collect();
}

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
///
/// # Examples
///
/// ```rust
/// match dynaport::random_registered_port() {
///   None => println!("no ports available"),
///   Some(port) => println!("{} is available", port),
/// }
/// ```
pub fn random_registered_port() -> Option<usize> {
  let mut ports = REGISTERED_PORTS_RANGE.clone();

  ports.shuffle(&mut rand::thread_rng());

  for port in ports {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the lowest registered port that is not being used.
///
/// # Examples
///
/// ```rust
/// match dynaport::lowest_registered_port() {
///   None => println!("no ports available"),
///   Some(port) => println!("{} is available", port),
/// }
/// ```
pub fn lowest_registered_port() -> Option<usize> {
  for port in REGISTERED_PORTS_RANGE.iter() {
    if is_available(*port) {
      return Some(*port);
    }
  }

  None
}

/// Returns the n lowest registered ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
///
/// # Examples
///
/// ```rust
/// match dynaport::lowest_n_registered_ports(5) {
///   Err(e) => println!("not enough ports available: {:?}", e),
///   Ok(ports) => println!("{:?} are available", ports),
/// }
/// ```
pub fn lowest_n_registered_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in REGISTERED_PORTS_RANGE.iter() {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
      ports.push(*port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

/// Returns the highest registered port that is not being used.
///
/// # Examples
///
/// ```rust
/// match dynaport::highest_registered_port() {
///   None => println!("no ports available"),
///   Some(port) => println!("{} is available", port),
/// }
/// ```
pub fn highest_registered_port() -> Option<usize> {
  for port in REGISTERED_PORTS_RANGE.iter().rev() {
    if is_available(*port) {
      return Some(*port);
    }
  }

  None
}

/// Returns the n highest registered ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
///
/// # Examples
///
/// ```rust
/// match dynaport::highest_n_registered_ports(10) {
///   Err(e) => println!("not enough ports available: {:?}", e),
///   Ok(ports) => println!("{:?} are available", ports),
/// }
/// ```
pub fn highest_n_registered_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in REGISTERED_PORTS_RANGE.iter().rev() {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
      ports.push(*port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

/// Returns a dynamic port that is not being used.
///
/// The port is chosen at random.
///
/// # Examples
///
/// ```rust
/// match dynaport::random_dynamic_port() {
///   None => println!("no ports available"),
///   Some(port) => println!("{} is available", port),
/// }
/// ```
pub fn random_dynamic_port() -> Option<usize> {
  let mut ports = DYNAMIC_PORTS_RANGE.clone();

  ports.shuffle(&mut rand::thread_rng());

  for port in ports {
    if is_available(port) {
      return Some(port);
    }
  }

  None
}

/// Returns the lowest dynamic port that is not being used.
///
/// # Examples
///
/// ```rust
/// match dynaport::lowest_dynamic_port() {
///   None => println!("no ports available"),
///   Some(port) => println!("{} is available", port),
/// }
/// ```
pub fn lowest_dynamic_port() -> Option<usize> {
  for port in DYNAMIC_PORTS_RANGE.iter() {
    if is_available(*port) {
      return Some(*port);
    }
  }

  None
}

/// Returns the n lowest dynamic ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
///
/// # Examples
///
/// ```rust
/// match dynaport::lowest_n_dynamic_ports(3) {
///   Err(e) => println!("not enough ports available: {:?}", e),
///   Ok(ports) => println!("{:?} are available", ports),
/// }
/// ```
pub fn lowest_n_dynamic_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in DYNAMIC_PORTS_RANGE.iter() {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
      ports.push(*port);
    }
  }

  Err(DynaportError::NotEnoughPorts {
    wanted: number_of_ports,
    got: ports.len(),
  })
}

/// Returns the highest dynamic port that is not being used.
///
/// # Examples
///
/// ```rust
/// match dynaport::highest_dynamic_port() {
///   None => println!("no ports available"),
///   Some(port) => println!("{} is available", port),
/// }
/// ```
pub fn highest_dynamic_port() -> Option<usize> {
  for port in DYNAMIC_PORTS_RANGE.iter().rev() {
    if is_available(*port) {
      return Some(*port);
    }
  }

  None
}

/// Returns the n highest dynamic ports that aren't being used.
///
/// Returns error if there aren't enough ports available.
///
/// # Examples
///
/// ```rust
/// match dynaport::highest_n_dynamic_ports(2) {
///   Err(e) => println!("not enough ports available: {:?}", e),
///   Ok(ports) => println!("{:?} are available", ports),
/// }
/// ```
pub fn highest_n_dynamic_ports(number_of_ports: usize) -> Result<Vec<usize>, DynaportError> {
  let mut ports = Vec::with_capacity(number_of_ports);

  for port in DYNAMIC_PORTS_RANGE.iter().rev() {
    if ports.len() == number_of_ports {
      return Ok(ports);
    }

    if TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok() {
      ports.push(*port);
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
    assert!(REGISTERED_PORTS_RANGE.contains(&random_registered_port().unwrap()));
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
  fn test_random_dynamic_port() {
    assert!(DYNAMIC_PORTS_RANGE.contains(&random_dynamic_port().unwrap()));
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
