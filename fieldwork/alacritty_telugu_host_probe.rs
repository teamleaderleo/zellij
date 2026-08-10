// Fieldwork execution-only tests for alacritty/alacritty at
// 1b2b36a64e88068ad02c95fad00ee2fad31c00bf.
// The owned workflow appends this module to alacritty_terminal/src/term/mod.rs.

#[cfg(test)]
mod fieldwork_telugu_host_probe {
    use super::{Config, Term};
    use crate::event::VoidListener;
    use crate::term::test::TermSize;
    use crate::vte::ansi;

    fn terminal() -> Term<VoidListener> {
        let size = TermSize::new(80, 4);
        Term::new(Config::default(), &size, VoidListener)
    }

    fn advance(term: &mut Term<VoidListener>, bytes: &[u8]) {
        let mut parser = ansi::Processor::new();
        parser.advance(term, bytes);
    }

    #[test]
    fn fieldwork_telugu_host_critical_cluster_uses_three_scalar_columns() {
        let mut term = terminal();
        advance(&mut term, "ద్యం".as_bytes());
        assert_eq!(term.grid.cursor.point.column.0, 3);

        // Ordinary CUB 2 therefore reaches column 1 on the legacy host.
        advance(&mut term, b"\x1b[2D");
        assert_eq!(term.grid.cursor.point.column.0, 1);
    }

    #[test]
    fn fieldwork_telugu_host_control_cluster_uses_two_columns() {
        let mut term = terminal();
        advance(&mut term, "వ్రా".as_bytes());
        assert_eq!(term.grid.cursor.point.column.0, 2);

        advance(&mut term, b"\x1b[1D");
        assert_eq!(term.grid.cursor.point.column.0, 1);
    }

    #[test]
    fn fieldwork_telugu_host_reporter_residual_uses_eleven_columns() {
        let mut term = terminal();
        advance(&mut term, "ఒక పద్యం వ్రాయి".as_bytes());
        assert_eq!(term.grid.cursor.point.column.0, 11);
    }

    #[test]
    fn fieldwork_telugu_host_full_reporter_line_uses_thirty_two_columns() {
        let mut term = terminal();
        advance(&mut term, "వసంత ఋతువు గురించి ఒక కంద పద్యం వ్రాయి".as_bytes());
        assert_eq!(term.grid.cursor.point.column.0, 32);
    }
}
