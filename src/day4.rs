use std::str::Chars;

const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn create_grid() -> Vec<Vec<char>> {
    INPUT.lines().map(str::chars).map(Chars::collect).collect()
}

pub fn solve_part2() -> i32 {
    let grid = create_grid();

    let y_size = grid.len();
    let x_size = grid[0].len();

    let mut matchs = 0;
    for y1 in 1..y_size - 1 {
        for x1 in 1..x_size - 1 {
            if grid[y1][x1] == 'A' {
                if ((grid[y1 - 1][x1 - 1] == 'M' && grid[y1 + 1][x1 + 1] == 'S')
                    || (grid[y1 - 1][x1 - 1] == 'S' && grid[y1 + 1][x1 + 1] == 'M'))
                    && ((grid[y1 - 1][x1 + 1] == 'M' && grid[y1 + 1][x1 - 1] == 'S')
                        || (grid[y1 - 1][x1 + 1] == 'S' && grid[y1 + 1][x1 - 1] == 'M'))
                {
                    matchs += 1;
                }
            }
        }
    }

    matchs
}

pub fn solve_part1() -> i32 {
    let grid = create_grid();

    let mut xs: Vec<(i32, i32)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, chr) in row.iter().enumerate() {
            if chr == &'X' {
                xs.push((x as i32, y as i32))
            }
        }
    }

    xs.into_iter()
        .map(|(x, y)| {
            vec![
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
                [(x, y), (x - 1, y), (x - 2, y), (x - 3, y)],
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
                [(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
                [(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
                [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
            ]
        })
        .flatten()
        .map(|word| {
            let x = word[0];
            let m = word[1];
            let a = word[2];
            let s = word[3];
            if x.0 < 0 || x.1 < 0 || m.0 < 0 || m.1 < 0 || s.0 < 0 || s.1 < 0 || a.0 < 0 || a.1 < 0
            {
                0
            } else if grid
                .get(x.1 as usize)
                .unwrap_or(&vec![])
                .get(x.0 as usize)
                .unwrap_or(&'0')
                == &'X'
                && grid
                    .get(m.1 as usize)
                    .unwrap_or(&vec![])
                    .get(m.0 as usize)
                    .unwrap_or(&'0')
                    == &'M'
                && grid
                    .get(a.1 as usize)
                    .unwrap_or(&vec![])
                    .get(a.0 as usize)
                    .unwrap_or(&'0')
                    == &'A'
                && grid
                    .get(s.1 as usize)
                    .unwrap_or(&vec![])
                    .get(s.0 as usize)
                    .unwrap_or(&'0')
                    == &'S'
            {
                1
            } else {
                0
            }
        })
        .sum()
}

const INPUT: &str = "XMMMSMMXMSXMXMAXSMMAXXXXSXXASXSASAMSXMAXAAMSMMMSXSAMXMMXMASMMMSXMXSAMXSASAMXXXXMMMMMMXMXSMXSMSMSMSXSSXSMSSMSAMMXXSAMXMXMXSMSMMASMXMAASMSMMMM
MSMXAASXMSASAAAXSASMSAMXMMMXAAMMMASMMXSXMXSAMAAMXSMMASXSSXMAAAAAMAMAXASAXMMSMMMXSAAASMMAMSAMASAAASXMAMAAAAXMAMAASAMSAMXMAAMAMAASMMMSXMAAMAAM
AAMMXXMXASAMXSSMSAMAXAMMSAXMMSMSSXMXSAXASXSASMXMAXASMMSAAASXMSSMMASMMMMMMAAAAXXAXMXMXAMAXMMSAMSMXMAXMXMMMSMSAMMXSAASASMMAXSASMMXAAAMMMSMSMXX
SSSSXASMMMSMAXXAMXMXMXMASXSAAAAXMXXXMASAMMSAMASMMSAMAXXMSMMMMAMASAXAAXASAMXSSMMMSMSSSMMSSMXMAXMMSSMMXMMAMAASXSXMSMMSAMXSAXAMMASXMMMSAXAAAASX
XAAMAMXAAAAMXMMMAAMAAAMMSXXMSSSMMSMAMAMAMAMXMAMAMAMMXMSXXMASMAXAMXSXMXMXXSAMAMAAAAAAXXAAAXXMMMMAAAAMMXSASMXMXMMMXMAMMMAMXSMAXAMAAMASXSMSXMAA
MMMMAXSMMSSMSXAASXSASXSMMASAAAAMAAAMMMSXMXMAMXSAMSASAMXAXSMMSSMMXMAXMSMMAMXMAASXMSMMMMXSAMSMSMMMSXXMAASXXXAXAAAMAMSSMMXSASXSMAXMMMAXAXAMMSMA
MXSSMMMSMMMAAMMMXAMXMXAAXAMMMSMMSMSXAXMAMAMASASASAASMAMSMSMAAMAXXMAXMAMMSSMSSXMMAXXXAAAXAASASMAAXASMMMSXMXMSMSMMASMXMAAMMSAXSAMSMMXSSMXMASXX
MAMAMMAAXSMSMSASXSAMMSSMMXSXAAXMAXAMXXSMMSSMMXMXMMMMMAXMAXMMSSMASMMXMASXAAAAMAMMSAMSMSMMMMMXMMMMMXMASAMXAMXMAAXSMXMASMMSAMAMMXXXASMAXMASXSSM
MXSSMMSMXSAAASASMMAMAAAAXXXAMMMMSASXMXAMAXAXXSMSXSMASMSMAMXMAAXAMASASAMMSMMMMXMAXAMXXXXAXMXMSXSXSXSXMASAMSSSXMXAAXAMXAXMXMMMSSMSAMMMMMMMAMAM
SMMMXMAMXXMMXMAMMSAMMSXMSMSMMXAAAMXAXSAMMMSMMMAAAXMAXAAMAMAMMSMXSAMMMASAXAXXXAMAMAMMMSSSMSAMSSMASAASMXMXMXMASXSMMSMMMSMAAXMAXAAMAXASAXXMASAM
AAAAXMMSASMXXMXMXSXSMXMASASAASMSSXMAMSXMAAAAAMMMMMMXSSMSASMSAMXXMXMXMAAMXSMSXSMMSXMAAXAAAXAMXAMAMAMMMAXAXXMMMXAXAAMAAAMSMMMMSMXMXSASAXMSASAS
SSMSXSAMXAAAXSSMAMXXAXMAMXMXXMAXXMMXMMSSMSSSSSXSMXSAAXAMMSAMXSAXAAMAMMSAMMASAMXMAXSASMMMMSSMSAMSSXMAXMSMSXXXXMSMSSSMSSXXASXXAMMMAMXMMMAMASMM
XMAXAMMMMMMMAXSMASXMMMMASXMXSMSMMMXAXSAXMAAXXAAXXAMMSMSMMMAMSSMSXMSSSMAXAMAMAMAMSXSAMXMAMAAXMAMMAASMMMAAMMMAXAAAXMXXXMASAMMSXSAASMSAAMXMAMXM
SMSMSMAASXMMXMMSMMAAAAMXMXMAMAAXAXSXMMAXMSSSSMSMMMSMMAMAASAMAMXSAXXAAXAMSMASXSXSXMMAMMMMMSSSSMSMMMMMASMSMSMASMMSMSMMSMXMXMAXXMASMAMXMXAMAXAX
AAXSXMSMXAXXAXXAMMSSMMSASAMASMMSXXAMXSMSAMAAXAAMAXAAXAMSMSASMMASXMMSSMXSXSXXAAXMASMMMXAAMAMXAAAAXAASXMXMASAASMAMMXMAXXXXSXMMAMSAMXMASXSSMMSS
MAMSXMAXSMMSXMSASAXAMASXSASASAXAMXXAMAAMXMMMMSMSSSSSMSAAMSAMXMASAMXAXMXMXMSMMMAXAMASASXMMAXSMMMSSSMSAMSMAXMASMAMXXMMMSAAXAAMSMAAXMMXSAMAXAAA
SMAMASAXAAAXMASAMMSAMXSASXMASMMSSMSSSMMMSXAAAMXXAAXAAXMSMMXSAMXXAXMSMMXMXMMMSAMMSSMMASAASXMMXMAAAMASAMXMXSMMXMAMSASAASMSMXMAMASMMASAMAMAMMXM
MXSXMMMXMMMSAMMXMXMMMAXXXMMMMAAXAASAXAXSXMSMSSSMMMMASXAAAXSXMMAXMMMAASMMAMAXASMAAAMMXMXMXAMXAMMSSMXSSMXSXMAAXXAMSAMMXMXXMMSAMAMSAMMASMMSSMSS
AAAXSASMXSXMMMMSMXMAMSMXMMAMMMMSMMMMSAMXAMXMAAAAAXAMXMSSSMMASMSSMAMSSMASASASMMMSSMMSMSSXSMMSMSAMXXXMAMXXASAMSMMXMXMXAXMAMXMASXXMSXSXMMAMAMAS
MXXMSAMAASXSAXAAMASMSAAAMSASXMAMMAMXSMSSXMAMXSSMMMSXXAAAAAMXMAAXMMMXMAXSMSXMXMAAAAMXMASXXAXAAMASMSSSMSMSAMXMMASXMMMSMSXAAAMXMMAMMMMXXMAAAMMS
MMSMMSMMMSASMMSXSAMXSMSMXAASXMAMSMSMXMAAASXSAXMAAAXAXSAMSSMSMXMXAMSAXMMSXXMSAMXXSMMAMMSMXAMXXSAMXAAAAAAMAMMMSAMAAMASXXXMXXMAXXSMAAMSSSSSSSXS
XAAMAMXASMAMXAAAMXSAMXMXMMXMMMSMXAAXAMSSMMASASXSSSMXMMAMXAAAMSMXXXMAXSAMXMASMMSMMASASXMASXXSAMAMMMSMMMMSAMSAMXMSMMMMMMAXASXXSAAXXXMAAXAXAAAX
MMXMAMAMXMAMXMAMXXAMXASMXXMAXAMXMSMSXMMAXMAMMMXXAAAASMMSSMMMMAMASMMMMMASXXAXMAMAMAMASAMXMXASXSAMMXAMAMMSASMXXSAXAMXAAXMAXSAASXMASXMMSMMMSMMM
MSSSXSAMXAMMAMSMSMMXMMSAASXSMMSAAAXMMMSMMMMSAAXSMSMMMAXAAAMSSMSAAMAAXSAMMMMMMMSAMAMXMMMSMMAMAXAXSXMSASAXMMASAMMSMMSSXSXSXMMMMAXAAAAAMAXXMAMX
AAAAMSMMSMMMSSMAMXMSASMMMSAMAXSXSMXMXAAAAXAMXSMSXMMXXSMSMMMAAXMMSSSSMMASXMSAXMAXSMSXMAXAASXMSMSXSAASMMMSAXASXMXAMMAMMSAXASXXMAMXSXMMSMMMSSMA
AMMMMXXAXXMAXAMXMSAMXXSAMXMSAMXAXXXXMMSSMSSMMXASAMXSXXAMASMSSMSAMXAXXSXMAAXMXMSXSXXAMSXSMMSAAAMASMMMMAASXMAMAXSXSMAMXMAXAMAMSXMAMMSAMAXAAAXM
SMMSXXMASXMSSMMAMAMMMASMXAXXMXMSMSMXSAAAMAXAXMXMMSAMXMMMASAMXXMAXSAMXMSMMMMSMAAASMSSMXXXAAMSMMMMMMSSMMMSAXXXMMXMMMSSXSMMMMMXAAMAMXMAXXMMSSMM
AMAMXXAMXAMXAXSXSAMAMXMASMMMMSMMXMAXMASMMSSMMMXSAMXSMSAMXSAMXXMAMXASMAAXAMAAMXMMMAXAAXAXMMMAXAAMXAAAXSXMXMMMSAXAAAAAASXSXAXSMXXASMXSMXAMAMXX
MMAXSASMSSMMMMAXMASXMSAAAAAAAAXMASXMMXMAAMXMAMAMASASASMSASAMXMSMSSMMMSMMMSSXSSXSMSMMSMSMSAMMSSSSSMSSMAAMXAXAAAXSMSMMMMAMMSMAAASMMMSXMMAAMMMM
XSASXAAAMMSAMXMMSMMMSSMMSSMMSXSSMSASXSMMMMMXMMAXAMXMAMAMXSAMXAAAXASAMXASMAMMMXAXAAXXAAXASMMXMAAAMAAMXXAMAMMASAMXAMASXMAMAMSMMMSXAXMAMMMMXAAA
MMSSMMMXMASMSASAAXAXXXXMAXMXMAMXASAMAMSMAAMXSMMSXSMMSMMMMMXMXSMMMAMXSSXAMXXMMAMMSMSMMSMMMMSMMMMSMMMMSXSASXXAXMXMXMASXMAMAMSXSXSMSMMAMASMSSSS
AXAMXSXSMAXXSAMSMMMSMSXSAMXAMAMAAMMMXMAXMXSASAAAAMAMAMXAASAMXXXSMSXXMASMAASXMMXAXAXXMXMSXMAMAMXMAMAXSAXAXSMSMMAMAMASASXSSSMAMAXAXSSMMMSAXAMX
SMXXMSASAMXAMAMMXMMAMXAAXAMXSAMXSXXAMSSSXAMASMMMSMASASXSXSASAXAXAMMSMXMXMSMMASMMSAMSMAMXASMSMSAMXMXSMAMAMMXMASASAMASAMAAXSMAMSMSMAXAASMXMAMA
AAMXMMMMAMSAMSMSAMSSMMMMMSAMXMSXXMMMXAAXMAMXMMMAMXAMMSAXAXAMAMSAAMASMXMXMXASAMXAMAAAMXMXMMXAAAMAMMSMMXMXMSASXMAMASMSAMMMMMSXSAMXMMMMMMAXSXMM
MXAAXAMXSXSAMMASMMAXXSXSAAXAASAMAMASMMSMAAXAAXMASMSSMMAMXMMXSSXMMMXMASXMMSXMASMSSXSMXSMSMAMMSMMAAXMXSXMASXASAMSMMAMSMMAXMAMXSAXXAXSMSMSMMMMM
XSSMMASXMASAMMAMAMMXXXASXMASXSASXSASAAAAMSSMSMSMSXXAMMMMXASAMMMSMMMMXMAAASASASMAMAMXAXAAAXMXAXMSSXSAMAMXSMMMMAXASMMSXSXSMXXASAMSAMXXAAAAMAAX
XAAASXMASASAMMSSSMXXMMXMAXAMXSAMXXASAMXSAMAMAAAMSMMXMAMXMAMXSAAAAAMAMSMMMSXMAXMASAMMSSSMSMMSMSMAXAMMSXSAMXXASAMAMXXSAMAMMMSMMMMASMSSMSMSSSSS
XMXMAMSAMAMXMAAMAAXSXSAXMMMMXMMSSMXMASAMMSAMMSMSXAXSSMSSMXMASMXSSXSAXXMSXMXMXMMMSXXAXMAXXAXAAXMAMMMXAXXMSSSXSAMSXSSMAMAMAAAMASMAMXMAMXAMXXAX
ASASMMMSMMMSMSMXMMMAASXSSXSXXXAAAMAMXMMSMMAXXMXSMMMAAMAMAAAXMSXMAMSMMAMMAMXMMMSAMXMSSSMMSMSSMMMSSXSMMMSAAAXMMXMMAMXMXMASMSSSSSMASASMMSSMSMSM
AXAMSASASASXAAXSXSMMASAMXAMXMMMSXSAMSAMXMSSMASAMAAAXMMXSSSSMAMSMAMSMSSMSAMXSAAMMMMAAAAXMAMXMAASXMASMAAAMMXMMMXMMMMAMASXXAXXXAXMASMMMAXXAXAXX
SMSMSMSASXMMMMMXASXXMXXAMAMMSXMXAAAXMAXXAAASAMMSSMMSXMAMAMXMAAAXAXMXAAXSXSAMMSSXSXMMMASMXMAXMMMAMSMMMSSMSMXSMAAAAXASAMMMSMMMMMMMMAAMSSSMMSMM
XAAASMMXMAMMMMAMSMAXMAXMSXMASXAXMASMMXSAMSXMSSXAXAAAAMMMSMXXMSSSSSMMMSMMSMMSAAMXMASXSMMAMSSSMSSXMAMMMAAAXXAASXMSSXAMAXSAMAAAAMASXMXSAAAASXMS
MMMMMAMXSMMSAMXMXAMXMASMXAMSMMMXSMXAXASMXXAAAMMASMMSXMMAMMMXAMAMXAMAXAASAMXMMXXAMAMXAAMSMAMMMAASXMSMASMMMMMXAXAAXMAMSMMASXSSXSMXAXMMMSMMMAMS
SASMSXMAMAASXSMSMXMAMXXXASMMXAXMMSSXMASAASXMMAAMXMMMMMMAXASXMMSMSSMSASMSAMMSMSSSSMMSAMMAMAXAMXSMSMAXMMAMMSXMMSAMSSSXXASXMAAXMMMSMMAXMAMASMMS
SASAMXMASMMXMMAAAMSMMSXXXMAXSXMXAAMAAASMMSXXXXMMAMAAAAMSMMMAMAXMAXAXMMMXAMAAAAXAAAAMASMXMSSMSXMAXSAXXXXMASXAMMAXXAXXXXMSMMMMXAAXXXXMSSSXMMAS
MAMAXASXSXSMSMSMSMAAAXMSAMXMSXMMMSSSMXMAXXXMMSXSASXSMXSAAXSSMXSAMMSMXAMSMMSMSMAMMMMMASAAAMAMSAMAMMMSSMAMAMSMMSSMMXMMMSAXMSMMMSXSMMSAAMXAXMAS
MASXMMAXMASAMAMAMXMMMMAAAMAMMASAMXAXXMSSMMSXAAXSASAMASMMSMAAAXMMXMAXMAMAAAXAAMSXXSSMAXMMMSAMSAMXSAMAAMAMMMXSAAAAMASAAMSMSASXAXAXAAAMMSSSMMXS
MASXMAAMMAMXMXMAMAXAAAMMXXSXSASAMMSMXXAAAAMMMMXMMMMMMMAAXMMMMMSXMXSSSSSSSMXSASXXMAXMMMXMASMMMAXXXAXMMSXMSASMMSSMSAMXMXXXSASMMSASMMSSXMAMXMAM
MAMAXAMXMXMXMXMAMAXSSSMXMXXASASXMAXMSMSSMASMXSXXMAMASMMMXMSMSAMASAXAAAXAAAXXXXMMSXXAMXMXMXMASMMMSXMSMSMASMMAAAMMMXSSMMMMMXMAXMAMAMXMAMMMAXAX
MSSMXASMXMSMAMSMSAXAMXAAAMMAMAMXMXXAAMXAXXMXAMXASMSXMAMSSMAAMXSAMMMMMMMSMMSXMAMXASMMSMSAMXSASMAMAXAAAMMAMXSSMSMAMXSXSAAXMASXMSMSASASAMASMSSS
MAAXSSMXAAAMAAAXXMAMMMSMSMMAMAMXXAMMMSSMMMAMXMAXAMXAMMMMASMXMMMASMXMAXXXXAXAXAMMASXAAMXAXAMMXXMAXXMMSMMMSAMXAAXAXXMAMSSMSASAMAMAAAXXASMSAAAX
MMSMMAASMSSSSSSSMSMSMAXXXXMAMAXMMMXSAAAAASXSASXSXXSAXSASMMSAMAMXMAAASMMMMSSMMMMMXMMMSSXSMMMXMSSMSMXMAXMAMAXMSMSSSMMSMAXXMASAMASMSMASMMXMMMSM
XXAAXMXMXMXAXAAXAAMAMMSMXSMSSXMAAASMMMSSMSASXXAMXMAAXSAMSAMXSSMMSMMMAAAAAAAASXMXAAAXAMAMASAASASAAAASMSAMXMAXMMXMAMAMMAMMAMXXSXMXAAAXAAXXAAXX
SSMSXXAXAAMMMMMMSMSXSASXAAAAAAXMMMSXAMMMMMMMXMAMSXMXMMAMMMSAMAAAXAXXMSMMSMSMMAASMMSMMSMSAMXMMAMSMSMSAMXXAAMSAMXSMMAMMAXXMXSAMXMSXMMSMSSXMSSX
AMXMXSSSSSSXAAMXMXMASASMSMMMSMXSAMXMAXAMXXAAXSAMXAXAMXAMAAMXSMMMMMSSXMXAAMAXMMMMMAAXMAMMXMXXMSMMXMAMAMMSSSMMMMMXXSSSSXSAAAMSAMXXSXAXAAAMSAMX
MSXMAAXAXAMXSMXAMASXMAMXXXMXMAASAMAMSSSSSSMSMMASMXMMXMAXMSMASXASMAAMASMSSXMMMAAMXSMSSMSMMMSAAMAMSMASAMMMAMXAAAAAAXMAMAAXMMXAMMMMSMAMXSMSMAMM
XMAMXMMMSMMMMSMXSXMAMMMMXMSMMAMSAMXSAXXAXMAXXSSMASMAMSAMSXMASXMMMMXXXMAMAASXSSMSAMXXAXXAAASMMMAMSXMSMSXSASMSSSSMSSXAMXMAMXXAXAAAXMXMAAXSMSXS
SSSMSXAXAAXXAMXMAMSAMXASAMMASXXXMASAMSMXMSAMMMAMAMMAXAMXAAMXXASXMSSSMMMMMMMAAAAXMAMSAMXMMXSXMMXMXAXXMAAMASAAMAXAAXMMXAXSSSMSSXSXMMAMASMMAXAA
XAMAMMSMMSMMSMMMSXMAMSAAMASAMMXMAMAMMAMAMMAAAMAMAXMXMAXXMSMXSSMMXAAAXXAAAXMMMMMMXAXMXMMSMXXMXASMSSMAMSSMSMMMMSMMMSASXMSMAAAAAXXAMMXSAMAMAMSM
MMMAMMAMXAMAAAAAMMMSAMMMXAMSSXXXMASXMASAMXASMSXXXSMMSSMSMAAMSXMXMSMMMSMSMSAAXMAAMSSSMSMAMMXSMMSXAAXXMAAMXMXAAMAMAMXMAMXMSMMMXASASAMMASMMMMAA
XSSMSMAXSAMXSSMXSAAAXXAXAAXAMMMSXMMASMSASMAMMMMMSAAMAMAAXMXMXAMMMXAAAXXAAMMMSMMMSAAMAAMMMMASAMMMSMMSXMASAMXMXXAMMXSSMMSMMXXXMMSAMXXSAMXMXAMS
MMAAXMAXMAMMXMASMMMSASMSXMMAXAAXMASAMAMAMMMMMAAXASMMAMSMXSAMSAMAAMSMMSXMSMSASAMSMMSMSMMSAMAXMMAAAAMXAXASXMAXSSMMMAXAXAAMMSSMAXMXSAXMXSASASXA
AMMXMASXSAMSAMXMAMAXXAXXASXSSMSSMAMSMXMMMMASMSSSMXSSSXXAAMAXAASMSMMAMXAAXMMASAMAAXMMXAAXSMSMMSXSMSSSSMXSASMXMASXSMMAMSMSAAAXSXAAMASMXSASMAMM
SASXMAMXMAASASXSMMAMSXSMXMAMMXMAMSAXMAXMASMSAAAXAXXXMAMMMSSMSMMAMMMMXSXMXSMAMASMSAMXXMXXMMXAAMAMAMAAMMMMAAXASAMXSAMXMXAMMSSMMMMMSMXMAXXXXMAX
AAXSMXMAMSMMXMXAAMMMMMMXAMXMMAMXMMAMSSMSXSMMMMSMMMMAMXMMXXAAXAMXMAASMMXMAMMXXXMMXXAXXXSASAXMMMAMAMMMMMSMSMXMMASXSAMSMMXMAMMXAAAXXMAXMMXSAXAM
MMMMSSMSMAAXMSXMSMMAMAXASMSSSMSAXMAMXXASAMXMAXAMSAMAMASXMSMMMXMASMMSAAAMMXSAXSAMXMMSMMXAMAXSXMASAMXMAAMAXXSSSMMASXMAMAXSXSASMXSXSSSXSAASAMSS
MSAMXMAXSMMMAXAXAAMXSMSMMAAMAMSXXXASAMXMAMAMMMXMMAXXSASAMXAAMXSXMMXSMMXSAAMAXSAMSAAMAAMXMAMMASMSAMMSMSMXMXAASXMAMMSXSMASAMMMMMMAMAMAMXMSAMAM
XSMSAMSMSMSSSSMSSSMAMAMXMMMXXMMMMSAMAXMXAMXMSAMXSMMXMMSXMMMMSASMSMXMMMAMMSMMMSAAMMMSSMSMMXXSAMMXAMAXMMMSXMSAMXAMMMMAAMAMXMMXSAMAMAMXMSMMAMMS
MXAMXMAAXAXAAXXMAMAASMMXMAMSMSXAAMAMMMXSASAAXMXMAXSAMASASXSMMAMMAMAAAMXSAMAMASMMSXMAMXXAAAMMMMSSMMSMXMASXAXASXMAMAMSMMMSMXSAMMSMMXMXAAXSXMXX
AMSMSSMSMSMMMMMMAMSAMAMMXAMAAMMMMSAMAAMSAMMSMMMMMASASMSAMAAXMSMSMXSSMSAMMSAMXSXAAMMASXXMMMSAMMXAAXAAXMAMMMSAMXMASMXXAMXMAAMASAAXSSMMSMXASMXM
SAAAAAAMAMXAAAXSAMMXSMMASXSMSMAXXSXSMSXMSSMMMMXAXMSAMXMXMSXMAMXAAAXMMMASXXAMXSMXSAMASMSMSMSAMAXAXSXSSMSSMAMXMAMMSMAMSMSXMASAMXXSAAXAAXAMMMAM
XSMXMMXMAMSMMSAAMSMAAMMMXAAAMXXXAMXMXXAMAXMAXMSMSXMASXAXXMASASMMMXSAMXAMMMSMAXAMXXMXMAAAAXSAMASMMSXAAXXAMSSSSXSMXMAMAXMASAMAMXXXMSMXSSMAAXAS
MXSXXMMSAMMASXMXMAMXSMSXSSMMMASMAMXXAMAMMSSMSXMAMMMSAMXXMSAMXSAAXAMXMMSXSAXMMMSMAMSSMSMSMMSAMAAAAXMSMMSSMXMASAAASXSSXSSXMXSAMSAMXXXAMAXSSSMS
SASMAMASMSXXMAXXSXSMAASAMXAXMASMAMMMMXAMXMAAXMMXMAXMASXMMMASASXMMXSMXMAAMAXAAAAMXMAAAAMAMAMMMMXMMMXAMXMMMAMMMSMSMAAMAXXASAMAXXAMMSMXMAMXAAAS
MAMMAMMMXAASXSMAMASMMMMMMSMMAAXMAXXAXXXXASMSMXSSSMMAXXMMASAMXXAXXMAXAMMSMMSXMSMXMSSMMMMAMXXSASXMAXXAXXMAMAMXAMXMMMMMXMXXMXSXMSAMAXMMMSMXSMMM
MAMSMSXSMSXSAAMXMAMAMSSMMAMXMSXMSMSASXXSXSMMMAXXAASMSMMSASAMSSMMXAMSMXAAAAAXXAMAAAXXMAMMSMMSASXMSSMMMXXAXAXMASAXAMXSAAXMSMXAXSXMMMAAAAAAXASM
MAMAASASAMAMXMASMSSSMAAXXXMSXMAAAAMAMAASAMXAMMMMSMMAAAMAASMMAAAXMSXMAMSSSMMSAASMMMMMMSXSAXAMXMASASAMXAXSSXMSAMASXMASMSAXAAMSMMASAMSMSSMXSAMX
SMSMSMAMXMAMXXXXAXMAMMMMMSMMASXMMSMMMMXMAMMSXSAXXXMXMXMMMMMMSSMMSAMASAXXAMXXMMMMSAAXAAXSAMSSMSMMASXMMXSAAXXMASXSAMASAMXXMSMAAAXMAMXAAMXXAMXM
SMMMAMXMAMXMXMXMMMMASMMSXSAMAMMAAXAXSSSMMSMMASXSMMSMSMSAXXXAXAMXXAMAMXMSMMMXXXAASMMXMXMMAMAMXAXAMMXSAXXMXMMXMAXXAMASASXSMAXXSMXSXMMMMSXMMSAS
MAXSASXMXMAMXAXAAXAAXMASASXMXXAMXSAMAAAMXSAMAMAMAAXAXASMSAMXSAMMSMMSMMXAMAMXMMMMMASASAMMAMAXMMSSSSXMMSMXMMXXMXMSMMMSXMAASXSXMMMMAMAAAXAMXSAS
SAMXMXMMASASXXSAMMMMMMMMAMXSMMSMXMXMMSMMAMAMSMMSMMSXMMMMSAXAMAMMAMMAASMSSXSAMXAXMXMAXASMSSMSAAAAMXMAAAMMMXAXMAMAASAMXMSMMMAXMAMSAAMSMSMMAMXM
MMSMMAMXAMAXAASASXXMXSXMXMAXAMXXAMAMAMAMASAMXSXAAXAMXMAMSAMSXSMSASMMXMAXXMMASMMSAMMSMXMAXAAMMMMMMAXSXMSAMAXXMXXAMXMASXMASMMAXAMXMSMAMMAMMSAM
XAAAMASMMSXMMMMAMMMSAMASXMSSSMSAMXASAXMXMSMSMASXSMAMMSAXMAMAAXASXSASMMSMAMSMMXAMXXAXMXSXMMMSSSSSSSXXAAXASMMSASMSXMSAXAMASMSASMSMAAMAXSASXSAS
MSSSMASXXAAMXSMAMAAMASAMAXMAXXMASMMMMSXMXMMAMMAMXMSMASMMSAMMSMAMMMAMMAMXXMAXXXMSSMMSMAMAMMXMAASXAXAMMMSMAAAMAMAXMAMMSSMAMXXAAMAASXMSXSASAMSM
MAAXMXMMMMMSAMMSXMMSXMASMMSAXMMXMMAAXMASAAXASXAXAMXMXSAMSXSAMXXXSMAMMXMASMSSSMAAAAAAMAXAMMMMMMMMAMXMXMAMMMMMXMAMMSMXAMMAXMMMMSSMXSAXAMAMMMAS
MMSMSAMXAAAMXMAXSMMMMXMMXAMXSMSMMSMMMMMSSSMMSMMXAMAMAXAMXAMASMMMASMSMXMAMAAAMAMMXMMXSSSMMXAAAAXMXMAMXXXSXMXAAMXMMMMMMMMSAAXXMAMXAAMMMMMMMSAS
AXMMXXSSMXSAAMXMAXSASAASMXSASAAAASXMXXAMAMAASAMXSSMSSSSMMXMXMAAAMMMAMMMAMSMSMSMSSSXMAXAXSSSSMXSXAMSXSXAMAMXSXXAASAMASAAXSSMSMSSMSXSASAXAXXAM
SAMXSMMASAXMASAMMMSASXSMMASAMXXSMMMMXSXSXSMMMAMAMAXAAXAASXMMMSSSXASASXSAXMAXXXAAAAMMMMMMMAMXAXXMXSAASMMMSSMXMMSXSASASMSMMXAMXAAXXMSASASMSMAM
XMAXAASAMMXAMXMMXMMAMXMAMAMAMAXMAMASXSAXAXMAXAMSMSMMSSSMMMAAAAMXMASMMASXSMAMSMMMSMAAXSMMMAMASMAXXMMMMAAAXAMXMMXASAMASXAMMMSMMSMMMMMXMXMAMXAM
MMMMMXMMSXXASAXXMAMAMXXAMXSAMXMXAMXSAMXMXMXMMXMXAXXXAAXXSSSMSMSAMXSXMAMMXMMSMSAMXXMXSAAASASAASXMASMSMXMSMXMSMAMMMXMAMXXXSAMMAXAXXMSSMSMSMMMS
AAAXXAAMXAASMMSXMASXSASMMMMXSAMSSMXMXMAXXMSMXSAMMMMMMSMXXAXAMXMXSASXMASXMASAMMSMAMASXXSMSMMAXMXAXXAAAASMXMAAMASXMASXMMXAMASMSSSMAXAAMAAXSXAX
XXMXSMSMMMMMAMXMSMSAMASXAMAAXXMAAXXSXMSMSAXAAXMXMAAAMXASMMMSMAMAMASMSAMMSMMMSAAXAMXXAXMXXAXMASMMMMMMSMSAAMSMSAMXSMSAMAXASAMAXAASAMSMMMMMMMMS
ASMMSXXASMMXAMAMAMMAMXMXSMMMSSMXXMMSAMMASXMMMSXSXMMSMXSXSAAAXXMASMXAXMAXAAAAMMMSMSSMMMMMSMMXASASXSMAMAMXMMMAMMMXSASAMMSXMASMMSMMXMAAAXAAMAMM
XXAAXAXXMAXSASXSASXSMXSAMXMSAMSAASAMAMMXMAMXMAAXXSAXMXMASMSSSSSMSASMSMSSSSMSSMXAMAAAMMAAAAXMMMMXAAMAMAMXSASMMXSAMXMMMAAXMAMXAXMSSMSMMSSSSMSS
SSMMMSMMSAMSASAXXXXAMAMAMXMMAAXSSMASXMMMXSMSMMSMAMMXAAMAMAMAMXAAXXMXAAAAXAXAAXAAMSSMAMSSSSSMASXMXMMMSMSAMXSAMMXMMXMAMMXMXAMXMMXAAMAXXAXMAXAM
XAAXAMAMAMAMAMMSSXSMMAMMMSXSXMAMXMMMMAMSAXASXAXMXMAMXXSAMXMAMSMMXXMXMMMSMMMSSMMSMXXXXXAAXAAMAMAXSXMXAMMASASMMSASAXMAMXASXSMSMSMSSMSMMMMSAMSS
SSMMMSSMMXSMSMSAAMSXSMSMAMASMSXMXXAAMAMSAMAMMXSAAMASMASMSMSSXXAMMXSAMXXAAAAMAAXAMXMMMMMMMSMMSSSMAASMMMSMMMSAXSASMXSASMAMAMAXAAAXAAAAAAXMAXAM
AAAXXAAAXMXAAXMMSMMAXAAMAMMMMMAMSSSMMAMMXMAMAMMXMSASMAMAAAXMAMAMMAMASMSASMMSSMMMMASAMAXAMXAAAAAMSMMAMXSAXXSXMMAMXMMASMXMAMMMMMSMMMMSMSSMSMMX
XSMSMSSMMSMSMSXSAAMXMSSMSASXXSAMMAAASXMMXMXXAAMAXMASMAMSMSXMXSXMMAMMMAMMMAAAAXMXSASMSAXSSSXMMMMMAAAMMAXMAMMSMMMMAXMMMMMMXSAMXAMXSMXXMXMXAAXA
MXASAMMAXXXMXMMMSSMXAXAAAMAAASXSSSMXMAASASXSXSXSXMAMXAXMXMASAAASXSSXMMSAMMMMSMAMMXSAMXXXAMXMAAXXMAMXMXSMMSAAXASXMSAMAMSMMSMSMMSAMXSXMASMSSMX
SMAMSMXAMSMSASAMAMXXXMMSMXMMMMAAAAXXMSMMAMXAXMAXSXXXMMXSASAMMSMMAAAXMMSASXXXXAMMSAMXMMXMAXSXMSSSMASMXXMASMXMMMMAXMMSAAAXXXMAXAMAMASAMAMAAAXX
XMSMXMMXMXAXAMSAMMMSXAXMASXXMMXMXMMSXMXMAMMXMMMMASXAMAASAMXSMXAMAMXMSASXMMXMMXSAMMSAMXASMXSAAAAMSAMXMAMXMAASMSSSMXMSMSMXAAMMSMXMMXSXMMSMSSMM
ASXAAXAASMSMSMXMAAAMSSMSSSXSXAXXXSASMXAXASMSMXXMAAMMMMMSAMXXMASMMSAMXXXMAMXAAMMMSASMSMASMASMMMSMMMMSXSAAMSMXXMAMXAAXAAXSSMMXAMAAXAMXAXAMAXAA
MMAXMSSMAAMAXAXXXMXMXMASAXAMXSAXSMASMSSSMSAAXMAMMSMXXMAMAMSMXAMXXMXMMXSMXMAXXXAXMASMAAAXMAMSMMAMAMSMAAMXMAMAXMMMSSSMSMXXAAXXAMSSMASXMMXMMSSX
MSAMXXMXMAMAMMMXSAMSMMSMXMMMAMMMMMMMAAXXMMXMMAAMAXMAMXMXAXSAMSSMSXAXSASMASMSSSXXMAMXXSMXMXXXMSAXAXAMXMMXSXSMXMAAXMAMXMXSXMMMSMAMMMAMXMAXXAMM
MMASXAXXXAMSSMAXMAMXASAMXMASMSAAXAXMMMSSMAXXAMXSAMXASAMMMMMAMAAAAMXMMAXSMMAMXAMMMMSMMAMXMSMAMSMSMSMMSXXMAAAMSSMXSSSMXMAMSMAAXAMXSSSSSSSSMXSA
XMSMMMMASAMXAMSXSAMSMMASXSASMMXMSXMASAAAMASMSMMMXSXMSASASMSSMSMMMAAMMSMAMMSMSAMSAXAAXAXAAXXMMMXAAAAXAAXSMSMAAAXAXAXAAMAXAXMASAXAASAAAAAXAASM
SMMAXXMXSASMMMMASMXMASMMAMASMSXXXAXAMMMSMASAAASXAMXAMXSASAAXMAAAMMXSAAXXMMMASAAXMSSSSMSMSMASXSSSSSSMMMMXXAXMSSMSMSASMSSSXSSXSXMXSMMMMMSMMASX
MASAMXMASAMXXAMXMXASMMASAMXMMXMASMMMSMSMMASMSAMMMSAMSMSMMMMMMSSMSSMMXSMMMSMMMMXAMXMXAAXMAMMAAAXAMXAXXAMAAMXMAMAAXMAXXAASXXMASASXMAMXXAXMAXSA
SAMASAMXSAMXSMMMXXMSASAMAMMMSXSAXAAAAMAAXXMXMAMAAMMXAAXAXASMMAXAAXXXAXMAMAMSASMXMASXMMMXAXXMMMMMAMSASASXMXMXAMMMXMXMMMMMSMMMMAMASAMSMMMMSMMM
MMSASASMSMSMXMSMMXXSAMMSSMAMSXMASMMSSSSSMSXXMAXMXSXSMSMXMMAMXAMMMSXMAXAASAMSASXASASXASASMSSXXMXXSAAASAXMMASMMSMAXXMSAAAAAMAAMMMXXXSXAAXAXAAX
SXMMSMMMSASMAMAAMAMMXMAAMXMXMXMAMXAXMAXAAXXAXMSSMSAMXXMASMSMSMMMXXAAMSSMSMXMMMMMMASXMMAXXAMSMMMMMMSAXMMAXAMXMAMMSXAAXSMSSSMSSSMMMMMMXMMXSMMS
AAXAXAASMAMSASXXMAAMMMMMMAMMMXSASMSMMSSMMMXMASAMXXASASMAMMAXAAAXMMASXAMMXXMASXAXMAMXMMSMMXMAXAAXMAMXASMXMSSMSASASMSMXMAMAXAXAXAASASXSMMMMXAA
SMMMMMMXMXMMMMXXSXXSAMXASXMSAAXXSAMXAMSAAXAMSMASXMMMASMSSSMSMMMSXMAXMXSAMXSAMSSXSMMASAAXSASXMSSSMXSXMMMXXMAMXXMAMXXXXSAMAMSMSMSMMAMAAMXAMMSS
MXMXXAAASASMSXSASAMXXXMAMMAMMSMMMMMMAXXXMSMSASXMMAXMSMAAXAMMMSMXAMAXASXXAXMAXAMMSAAAMXSASASAAMAXXXSXMAMMXMMSXMMAMMSAXMAMMXXAAAASMAMSMMMXSAAM
SAMXSMMSAMXSAAMMMMMXXMASXMMMAXXAAAAXMXSMMAMSASXASMMMMMMMXSAMSASXXMAXSXMASXXMMMXAMAMXSAMXMAMXMMAMXXMASXMMASASMXSASAXMASXMMAMXMMASMMXMASXAMMSS
SAMXSMMMXXXMMMMMAAMMXMAMSSXMXSMSSSMMSAAMSAXMAMMMMAMSASAAAAAMSASMXMSMMASAMASMSSMSSXSAMXSSMXMASMMMSMSAMAASASXSAMXAMMSMMAXXMASXMMMXAMAXAMMXSAAX
MXMAMAXAMXXSAMASXSSMAMXMASXSXSAMMXAAMMMMSXSMAMAMSAMSAXMMXXMMMAMAXMMAMAMAMXMAAXAAXMMXSASMMSSMAAAAXXAMAXXMAXAMXXXAXMAMXMAMXASAXXMSAMSMASAMXMAS
MAMSXMMSSMXXASASAAAXSMSMMMMSAMXMAMMMMMSXMMMMXXSMXXXMSMSSXSAMAMMMSASAMAXXAMXMMMMMSMXAMXMAAXAMSSMMSSSSSMXMXMSMAMXMMSXSAMMSMMSMSAASAAAXMAMXXMAM
SASMASAMAXASAMXSMSMAMASMMAAMXMXMAMAXAAXAXXMASXMMSMSAXXAAASMMASAXMASMMSXMAMMXXXAAAMMMSXSMMMXMXXXXAAMAXXAXXAAMXSAAXAAXXSASAXXAMMMMMMMXSAMXMMAM
MSXXXMAXSMMAMXXSMMXMMAMASMMSASAMXMSSMSSSMAMAMAAAAAMMMMMSMMASASXMSXSAAAXSAMXMMSMMSSMSAAAMAXSSSMSMMSMMMMMMMSXMASMSMMSMMXASMMMMSMSXSAAXSSMAXSAS
SXXAXSSMMXSXSXXMASMXMASAMAXSXMAMXXAAMAAAXXMMSSMMXSAMSAMAMXXMAMAMXASMMSMXAMXXAAAXXAMXMMMMAXXAAAXMXXAXAXXAXMMSMSXMMAMXXMXMAXXXAXMASXMMMXXAMXXX
AMXMMAMXMXMASMXMAMAMMAMMSMMMMXAMXMSSMMSMSMMMAXAXMMAMMAMASXMXMSAMMMMXXAAMSMSMSSSMMMMMMXXXMSMAMSMSAMXSMSSXSAASMMAMMAMAXAMSMMMXMXMXMMSSXMXSXMSM
MXAMMXMASXMAMAXMXXSXSAMMAXAXASMSSMMAAXMASXAMAMXMXSAMSSMASAXAXSMMSXMMSMSMXASAMAMAASAMXMMAXAAAXXAMASAAAAAASMMMASXMSSXMXMAAMASAMXSAMXXMASMXAMXA
XMSMMASAMAMXXSASMMMAMASXMSAAXAAAMXXMMMMAMMXMAXMAMXXXAAMAMMMMMXMAMASAMXAAMAMSMAMSMSAMSAMAXMSAMMASAMMSMXSMMSXSXMAMMXAXASMXSASXMASASMXSAMXSAMMS
MXAXSAMXSAMXXAMAAXSAMSMXASASXMMMMMMAAAAXMASMMSAAAAXMSSMMMXAXXAMMSMMASXMXMAMMMXMAASAMMAXXMXAMXSXMASAXXMMXMAXSMMXSASXMMSAAMXMAMASMMXAMAMAXAMAM
SMMMMASXSASMMSXSMMMAMXASMMAMXXMSASMSMSXAMXMAAXMXSXSAMXXMSSXSSMMAAASXMAASMXSAASMSMXXXSSMMMAASXSASMMMMSAMXMSMSAAAMXMAMXMMMMMSMXAXMSSSMXMSSSMSS
AAXAMXMASAMMAMAAAASMMMAMASXMXMMMASAMAMXXMAMMMSMMMASASASAAMXAMXAMXMMSAMSMSAMMXSAXAXSMMAAASMAMAMAMXAXAXSXMXXASXMXSASAMAMAAAXAMMMSMAMAAXSAAMXMS
SXMMSMMXMAMSSMSMSXXMASMSXMAMMAXMAMAMMMMMXAXAXXMASASXMAMMAMSSMMSMMAASXXAAMMMSXMMMMXXASXSMSXAMXMXSSSMMMXSMXMMMMMXSASAXXXXSSSSXSAAMASXMMMMSMAMM
MASXAXMAMAMXMAAMXMMSAMAMASXMAXMMSXMMSAMASMSMMMMXMXMAMXMSMMMAAAAMMMMMMSMSMAAXASMAXAMXMXAAXMXSXSMAAXAXAMASXXMAAMMMMMMSSSXAMAXAMSXSASAMXXXAXXMM
SAMXSMSAXASAMSMSASMMASXSXMAXSXSAMASMSAMMAAAXAAMASAXMMAXXXSMSMMXSAMASMXAAMMSSMMMSMXSAAMMMMMXMAXXMMSMMMSAMXMXSXSAXAXXAAMMSMMMSMXXMASXMASXMSSSS
MSMAMMSMSMMAMMASASASASAMXMMMMXMASMMASXMMMMMXXMSASXSMSSSMASAXXMAXXSASAMSMMSAMXAXAAMMMMSMSXSAMMMXAMXMAAMAMMAMAAMMSMSMMSMAXAAAASAMXXXXMSMSAAAAA
SAMASAXMAMASXSAMASAMXSASXMSAMAMMMAMMMASAMXSSMXMMSMAXAMAAASAMMMMSMMASMMMXMMSSXXXSSSMSXAMAASXSXAXAXAMMSSMXSSSMXMMAMAAMXMSSSMSSSSXMSMXMAAMMMMMM
AMMAMAMXMASMMMXMXMXMXXMXMASASXSSSMSXSMMAXAAAMXAAXAMMXMXXXMAMXAXAAMAMAAXSXAMMXSAAAXAMSSMMAMASASMMMMAAMAMMXAAXMAMXSSSMXAXMAMMAMXSAAAAXMMMAMAXM
SXMASAMXXMASAXAMXMASXXMAMASAMAAAMXMAXASMMMSMMMMMSSXMSMAMSMSMSAMMSMAMMMSAMMSAAMMMMMAMMMAXMMAMMMMXAAMXSAMAMXMMMSAMXXXAMXMSMMMAMAMSXMSSSSSXSMSS
XMAXAXSAAMAXSSSSXXMAMSMMMAMXMMMMMAMXMASAXXMASXXMXAAAAMAMAAXXXASAMMXASXSMXXMMMXSASXMMMSMMSMAMXAXXSSMMSMSSMMMAXAAXSSSSMSMXAAXAMXMMSAMAAASAMMAA
MSMMMXMASMXSAMXMXSXSASAASMSSSXSSSMSAMAXMMXMAXMSMMSMSXSAMXMMSXXAAAXXMXAXXMMASAASASXMSAAASAMMSSMSAAAAAMXAAASASMMSMAXAAAASMSMSAMXAAMXXAMXMXMMMS
AMASAAAAMXXMASXMAMAMASMMMAAAAAMAAAAAMAXSMSMSMMAAAXAXASASMSSXMMSMMSSMMMMMSAAMMXMXMAAMMSXSASAAMAMMSMMSSMSSMSAXMAMMMMSMMMAXXMAAMSMXSASXXMASXSXX
MAAMSAMXSXMSAMXMASAMAMAAMMMMMAMMSMSXMASXAAAAXSSMMMXMASAAAXMASAMAAAMXASAASMXMXASXSMMXMMMSAMMSMAMMMMXMAXMAAMAMMASAAAAAAXMXMASAMXAXMASXAXAASMAX
AMSXXXMXAMAMXMSAMXMSMSSSSMSAXSMMXXMASXAMMMMMMMASAMXXXMMMAMSAMMSMMSSSMSMXSSXMSXSMXSAXSXMMSMAAMXSAMXXSXMSMMMAMMMSXMSSSMSXSAMXAXXSXMSMMXMASAMXM";
