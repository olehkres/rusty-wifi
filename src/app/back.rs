use crate::wifi::AccessPoint;

pub struct AccessPointGUI {
    raw: AccessPoint,
    points: Vec<[f64; 2]>,
}

impl AccessPointGUI {
    pub fn new(ap: AccessPoint) -> Self {
        Self {
            points: Self::build_points(&ap),
            raw: ap,
        }
    }

    pub fn raw(&self) -> &AccessPoint {
        &self.raw
    }

    pub fn points(&self) -> &Vec<[f64; 2]> {
        &self.points
    }

    pub fn scan() -> Vec<Self> {
        AccessPoint::scan()
            .into_iter()
            .map(AccessPointGUI::new)
            .collect()
    }

    fn build_points(w: &AccessPoint) -> Vec<[f64; 2]> {
        let start = (w.channel() - w.bandwidth() / 20) * 30;
        let end = (w.channel() + w.bandwidth() / 20) * 30;

        (start..=end)
            .map(|x| {
                let x = x as f64 / 30.0;
                let a = *w.bandwidth() as f64 * -0.05;
                let b = *w.signal() as f64 / 10.0;
                let c = *w.channel() as f64;
                let y: f64 = a * b * ((x - c) * (x - c)) + b;
                [x, y]
            })
            .collect::<Vec<[f64; 2]>>()
    }
}
