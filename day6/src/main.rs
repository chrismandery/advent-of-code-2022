use anyhow::{anyhow, Result};
use std::collections::HashSet;

fn get_marker_end(input: &str, window_size: usize) -> Result<usize> {
    // Input is plain ASCII anyway
    let b = input.as_bytes();

    for (i, w) in b.windows(window_size).enumerate() {
        let mut uniq: HashSet<u8> = HashSet::new();
        uniq.extend(w);

        if uniq.len() == window_size {
            return Ok(i + window_size);  // Window does not contain a duplicate -> we are done
        }
    }

    Err(anyhow!("No marker found!"))
}

fn main() {
    let input = "mgtgddtfdtffzvznvnrncrrbqqhlhhffzqqzpqqthhrhhfphfphhcppcddnwdnwwtmwttfvvthvvrrbvbmvmssrlslfslflppblllwrlrzlldwdllqblqbqbsscmsmwwffjpppnlnhllbblvbvsbbzvzrzzsmsjsddfpftfvtffgjjfzjfjqfqjfjsscvcccgttgtzgzmgmtmbbwzzjqzzdfzfmzmzfzwzvwvggqcqrrcwrcrzrccqcwwbgbqqwdqqzjzsjjwbjjssmmcfcbcddlhhtltmtlljffvjffhghmggmvvfgfqgfgppnpllmvmfvvzjjzrztztvvstsvvppqdpprjjmtmjtmjjdrdcrdccgsccnsccqsqzszqsqgqwggbhbllvclljrrlrqlljtjcjjlrlhrhjhjnnnpllwtwstttlnlqnlnmnqnpqpbqqbgbzzrhzrhzhrzhrzhzshhqvqgqgbbcqccqmcqccvgccrwrgwrgrdrhhbshbhwbhwbhhvthvttfrrqsstqssqmmpnpwpfpcffcdchhrsshrhggtcttmrrhvvjfvjvvclvllmqmvvhddrdjjhdhvhlvhlltlstltffbbqbwqbbbnsbnbwbssjwsjsfjjsjwwzttqzzsdspprlrblrltrrfrsfffwqwpwddddnqqtbtwwhwpwdwmmcrmrsmmwppjzpjpcpdpjpdpdqppmjjlqqjfqqhgqhhbddtccthhwjhhlfftvtppwzpwzpznpzpqpgqpgpnpdndnbnddqrrjdjwwdmmtnntvnnrhrfhfrfwfvwffmnfnlfldfdjjwgwqqwwsslrrvhhrqqsfqfllrmrqmrrbppwjppmlmggvppdhppspjjzljjrzzrlzrrlldllvlpvpfprfprrhdrdlllpqqfhqqhchzzzwpwjpjjgzzwqqtqdqbdbdgggbrgrzzznwwbvbnbpnbpprnrvvfvsscncrczcbchhjqhjjzrznndwnnvttmtthssgvvbvfvtvptpthhzggnjjhrjdqzjbtfpqdtwtmgnngqdzhdrfzqvcqggmcdbsdrdrmgqhmvfvdgbvrnlbhfsbpjhwgzfndqgcjdbpsffcslfcltsbclspdjhscqrncfrjrbjfzspccshtrdggjbhthrrhgnjvsptfnjvjvhhdjfbtfgpfgszhhbcvzplclrnsrpffpjhbthnfsfflqphhjjdpcfwzhfdpnsftrnfhrdhndlrnfrnvprtvnmgclzlrdjrzdcllvlwdlrcfbsgcbwcnbvjztzfsgcgqlmgcbsgwbbrmrcthfpvmbfvtbhqstccfntmphqpjwpbcdpnffqpszlnqdcqtfhvlvpgdpljvcschdtpvcswfzcbpqdhfjzzdjvgldspcvlnfnwffhjzdnbmjjtnrqlgnggsvdltnrpcfwqvphtsmrfzhflwjjbnpwlzhhmdnpqptgcjnrrgcnhwllqsbsjjvzmqsghlzvhdfbrnfhrqjswrpgcctsqvdwzgpqdssfmtgwvsznlbhsgppwdhhtjmscjfrjdgflwcrlbsfwrnvtnmcwpndhtttgqfvmvmfnwdrrvgmgdqlqvvlphwzgmwcphjvcfsqbbwttntmgvfmlmctggmtlwtmfsmczbgdvbsjstzgflnjplgrlhbbgldlchwmhclzbcwpqzlzbjzbplnvpbzjhmwfmrfnwlnsvpzhrgjdpqvnjtbfjfsvdqcfwdjftsmfqdrqllwlbnbmgtswrhbtbqlchznbgnphgntrtwbtmsjtphhqpbngwmmsdnsdqcctrsrzbrtpwtvhvqbrjldfldllpvspthdhdljfvjzcjsltwflscfqsrvzhgvzhqnnjwdwdtnsvgchzrnbzfscvsmrmqsqjmrjjdhtspbzpqtqqbfbzrddwqzwpqjbpbbbghlwmzhqvqdwwwwvltvvcpgzlwzvmqzfcgnjpjnpgsccvzpnzjwwnnjrcpbvwljfrjqzwsrvdmqwwfpldqcdwlchvggclmwnbhlrlzvsrtrqmzchqfqfhqfjgqsfvclnchdnnvdbqpcddnldggwrpbgrwwtssfndhrhnwtqmgrwpggntlqmfgbzjhwwsclvfmwgzzfrsccdfddntnlldpnwzhnzlssnnfbvjjhnrvclmphgfpvnwjzznbvgqnpljcrjpndgrlbdzsbfrrrfztbqcbphlppwcvhmrrmtrlvfjcddtznlmflrpsclgjpqczwrptfsccmdpzfvwnfsvshcnzrjrmstrslhgtrsmgplvcwptfqrgzgwhvtvrqlrjpcbztgtfwpnzqpmctvpdlgrtzzlsmgnftqvtvcmndspjqbdnmrttwhdrncsntntmrwjrqstdrptnhbqgtlqsdqfmbjtvgstndlvndqqsbqvcghwwjdzpszrsfpdzvnmnbzngczndtwtmprbzjdzbthslttzwwfptbphqwczsrqcbcbqnhbtcpjpbcqpjgjmhmfnggcbvctslpmqrpqzbcfrcgzmzpbpwzsjlrmpfzhgnnbqfrbslrfsthgtmsdfhzgdmjwwsgcdptssmbvffhlmfvwnmbpnzbvpsvnwsvsgrcmhpclwsbvtfqstnpzvgmgfcrmjhbccwcptssjhbfmzsqljjcrnnszvffzfwgcpnqrtjnqdltwnbglwlwpschvqwfdztvcwsqtwmgwccgsqbsvlwdhlnqphwtcmdpvvrqfwmlbptbvghvjntqbcsqjspwnmvdqcfbqzqchhhwqgdcmdhfvtzprscpshpbmzhwsznlpvzrwvmhtqsclzffgnvvrfbzmvqmnrrzjbmhdbspjprrmflgrwhnhcqpczchpnrnfjgdlnlrnzwnvjpmzgpfzspwmfnwcrrdczdhtnscmwqwqbcrdrsndpwbdvpgpbpsfzbmvjlsrdcgnwgrvmjnzlpnwtcrmnfcqgmlnhqbwlrnzlbdrnzfhnqddsfmnhnrrrdjgqprmgvrnhzrlccjthhfzdbltgrbrjpmbhvgrlwngdlfsljhfvwhvpmltdfnzwzcgzdpppnzcnpjttdgpzzqppnfzlmhrngbmcmshtgzbjllwstdbnmmwlrlllgfgshvcsjbpnggzrvvmvdqhjhvhmmpvrdqbrfpdtcdbqrvwhdrtqgftnwwzrcgzwmwjmdgmfswqwlgmvmvhscjmzshtbzmfmbqtbsjppzbczwcqpqhhqdggcntdchjgwsvfnzfqdzvhpnwbjhbqnldzbzmctcdqgjsmbqdzmmtjzvqzdqzsfpmncdmqlnpsrwcznbtzqtbcwwdqjftcdmmwdjdnwvpchffsmqmmwvqfgcnfhbjsttwnwppssmvrrhrbqwsncpfnbfggdqjwbgtvgtwsmlqbwzlghnzhjwphswjtbtptmhlzhvvrwqqcgwnmcqtcjlndwgjrpschhhsmrvvwtrjplwrtswhrjlgjhzgzrjhsbrjhtgnmfdvbjlntcrphsnmdcjzgwtvgldrfpcfgpzlgsfthdmpbnhmlsbnbqzpqvzzmvswbbnbtzvbsznqdgqlbbwzhjrzndltfgswtszsmmrhrcrcrcpgtqfcrmjrtflsbcbbmrsrfgnsrmbrpcvfpmqtmbrbbqtzrjntnvbvwjwqmwmcvmzccmwcnhrfpgghlqczcfszfhqgrdnfpnrrzpzbnjqjtvbglvqlhpstpzzcwrdgfhghqtsgzgsmgnpgvbsvsjtnwbvtqpcfdvhnjjvwjwglplthmghrwpmsgbdbfpvqsmsdvjgchlnlnczlzczqmjsnpgrgqgndwzdtlmmgzjpqvbqmcmhnhpqvpjjsftctwsrfmhrlctrvhczjbfsvqnshmchdsrmlrlqdnfsvhlblwghsdnrtwnpdtqgczmghqcmfzvsgqvrngjvbjsvnpzvpsplhvndvqpjjrtmrqscjrhvdmqcgwjmrgsdmgswgnbpdtgvvbrzrcwtvvwhpmcqwdtsmwwfgdpdrjsbvtbdvbhwftqznpssnsnjnclblslfgz";
    println!("First packet marker after character: {}", get_marker_end(&input, 4).unwrap());
    println!("First message marker after character: {}", get_marker_end(&input, 14).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(get_marker_end(&input, 4).unwrap(), 7);
        assert_eq!(get_marker_end(&input, 14).unwrap(), 19);
    }

    #[test]
    fn example2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(get_marker_end(&input, 4).unwrap(), 5);
        assert_eq!(get_marker_end(&input, 14).unwrap(), 23);
    }

    #[test]
    fn example3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(get_marker_end(&input, 4).unwrap(), 6);
        assert_eq!(get_marker_end(&input, 14).unwrap(), 23);
    }

    #[test]
    fn example4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(get_marker_end(&input, 4).unwrap(), 10);
        assert_eq!(get_marker_end(&input, 14).unwrap(), 29);
    }

    #[test]
    fn example5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(get_marker_end(&input, 4).unwrap(), 11);
        assert_eq!(get_marker_end(&input, 14).unwrap(), 26);
    }
}
