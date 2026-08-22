pub mod validadores {
    pub fn cpf(cpf: &str) -> bool {
        // 1. Extrai apenas os números da string (ignora pontos, traços ou espaços)
        let numeros: Vec<u32> = cpf.chars().filter_map(|c| c.to_digit(10)).collect();

        // 2. Um CPF válido deve ter exatamente 11 números
        if numeros.len() != 11 {
            return false;
        }

        // 3. Rejeita CPFs com todos os dígitos iguais (ex: 000.000.000-00)
        // O .windows(2) cria pares para checar se o atual é igual ao próximo.
        if numeros.windows(2).all(|par| par[0] == par[1]) {
            return false;
        }

        // 4. Cálculo do Primeiro Dígito Verificador
        // Multiplica os 9 primeiros números por pesos decrescentes (de 10 até 2)
        let soma_1: u32 = numeros[0..9]
            .iter()
            .enumerate()
            .map(|(indice, &numero)| numero * (10 - indice as u32))
            .sum();

        let resto_1 = soma_1 % 11;
        let digito_1 = if resto_1 < 2 { 0 } else { 11 - resto_1 };

        if digito_1 != numeros[9] {
            return false;
        }

        // 5. Cálculo do Segundo Dígito Verificador
        // Multiplica os 10 primeiros números por pesos decrescentes (de 11 até 2)
        let soma_2: u32 = numeros[0..10]
            .iter()
            .enumerate()
            .map(|(indice, &numero)| numero * (11 - indice as u32))
            .sum();

        let resto_2 = soma_2 % 11;
        let digito_2 = if resto_2 < 2 { 0 } else { 11 - resto_2 };

        // 6. Retorna true se o segundo dígito também bater
        digito_2 == numeros[10]
    }
    pub fn cnpj(cnpj: &str) -> bool {
        // 1. Extrai apenas os números da string (ignora pontos, barras ou traços)
        let numeros: Vec<u32> = cnpj.chars().filter_map(|c| c.to_digit(10)).collect();

        // 2. Um CNPJ válido deve ter exatamente 14 números
        if numeros.len() != 14 {
            return false;
        }

        // 3. Rejeita CNPJs com todos os dígitos iguais (ex: 00.000.000/0000-00)
        if numeros.windows(2).all(|par| par[0] == par[1]) {
            return false;
        }

        // 4. Cálculo do Primeiro Dígito Verificador
        let pesos_1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

        // O .zip() une a lista de números com a lista de pesos formando pares perfeitos
        let soma_1: u32 = numeros[0..12]
            .iter()
            .zip(pesos_1.iter())
            .map(|(&numero, &peso)| numero * peso)
            .sum();

        let resto_1 = soma_1 % 11;
        let digito_1 = if resto_1 < 2 { 0 } else { 11 - resto_1 };

        if digito_1 != numeros[12] {
            return false;
        }

        // 5. Cálculo do Segundo Dígito Verificador
        let pesos_2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

        let soma_2: u32 = numeros[0..13]
            .iter()
            .zip(pesos_2.iter())
            .map(|(&numero, &peso)| numero * peso)
            .sum();

        let resto_2 = soma_2 % 11;
        let digito_2 = if resto_2 < 2 { 0 } else { 11 - resto_2 };

        // 6. Retorna true se o segundo dígito também bater
        digito_2 == numeros[13]
    }
}
