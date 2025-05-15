pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[allow(dead_code)]
fn divisores(valor: i64) -> Result<Vec<i64>, String> {
    if valor <= 1 {
        Err(format!("{} <= 1 não é um valor válido", valor))
    } else {
        Ok((2..valor + 1)
            .filter(|x| valor % x == 0)
            .collect::<Vec<i64>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}

    #[test]
    #[should_panic]
    fn zero_is_not_valid() {
        divisores(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn um_nao_eh_valido() {
        divisores(1).unwrap();
    }

    #[test]
    #[should_panic(expected = "-10 <= 1 não é um valor válido")]
    fn negativos_nao_sao_validos() {
        divisores(-10).unwrap();
    }

    #[test]
    fn tres_divide_tres() {
        assert_eq!(divisores(3).unwrap(), vec![3])
    }

    #[test]
    fn dois_divide_dois() {
        assert_eq!(divisores(2).unwrap(), vec![2])
    }

    #[test]
    fn divisores_de_quatro() {
        assert_eq!(divisores(4).unwrap(), vec![2, 4])
    }

    #[test]
    fn divisores_de_vintequatro() {
        assert_eq!(divisores(24).unwrap(), vec![2, 3, 4, 6, 8, 12, 24])
    }

    #[test]
    fn usando_results() {
        assert_eq!(divisores(4), Ok(vec![2, 4]));
        assert_eq!(
            divisores(-10),
            Err(String::from("-10 <= 1 não é um valor válido"))
        );
    }
}
