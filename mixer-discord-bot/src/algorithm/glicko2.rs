pub fn update(r: f32, rd: f32, volatility: f32, r_opponent: f32, rd_opponent: f32, scale: f32, score: f32) -> (f32, f32, f32) {
    let tau = 0.2f32;

    let mu = (r / scale - 1500.0) / 173.7178;
    let phi = rd / 173.7178;
    let mu_opponent = (r_opponent / scale - 1500.0) / 173.7178;
    let phi_opponent = rd_opponent / 173.7178;

    let g = 1.0 / (1.0 + 3.0 * phi_opponent.powi(2) / std::f32::consts::PI.powi(2)).sqrt();

    let e = 1.0 / (1.0 + (-g * (mu - mu_opponent)).exp());

    let v = 1.0 / (g.powi(2) * e * (1.0 - e));

    let delta = v * g * (score - e);


    let a_init = volatility.powi(2).ln();

    let f = |x: f32| -> f32 {
        ((x.exp() * (delta.powi(2) - phi.powi(2) - v - x.exp()))
            / (2.0 * (phi.powi(2) + v + x.exp()).powi(2))) - ((x - a_init) / (tau.powi(2)))
    };

    let epsilon = 0.000001;

    let mut a = a_init;
    let mut b = if delta.powi(2) > phi.powi(2) + v {
        (delta.powi(2) - phi.powi(2) - v).ln()
    } else {
        let mut k = 1.0;
        while f(a_init - k * tau) < 0.0 {
            k += 1.0;
        }
        a_init - k * tau
    };

    let mut f_a = f(a);
    let mut f_b = f(b);

    while (b - a).abs() > epsilon {
        let c = a + (a - b) * f_a / (f_b - f_a);
        let f_c = f(c);

        if f_c * f_b <= 0.0 {
            a = b;
            f_a = f_b;
        } else {
            f_a /= 2.0;
        }

        b = c;
        f_b = f_c;
    }

    let sigma_prime = (a / 2.0).exp();


    let phi_star = (phi.powi(2) + sigma_prime.powi(2)).sqrt();
    let phi_prime = 1.0 / (1.0 / phi_star.powi(2) + 1.0 / v).sqrt();
    let mu_prime = mu + phi_prime.powi(2) * g * (score - e);

    let r_prime = (173.7178 * mu_prime + 1500.0) * scale;
    let rd_prime = 173.7178 * phi_prime;

    (r_prime, rd_prime, sigma_prime)
}