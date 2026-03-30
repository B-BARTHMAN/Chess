pub struct PrettyBitboard(pub u64);

impl std::fmt::Display for PrettyBitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bb = self.0;

        writeln!(f, "  A B C D E F G H")?;
        writeln!(f, "  ----------------")?;

        for rank in (0..8).rev() {
            write!(f, "{}| ", rank + 1)?;

            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (bb >> square) & 1;

                write!(f, "{} ", bit)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
