use std::collections::VecDeque;
use std::io::{self, BufRead};

/**
 * Problem
Welcome to the Grand Banquet Dilemma! Picture a gathering of folks connected by a web of relationships. This web of relationships is given as a connected tree with N vertices depicting people and N−1 edges representing relationships (for each edge the corresponding two people know each other).
You're on a mission to throw an unforgettable party, but there's a twist – you've got two banquets, and you want to ensure that no two people who know each other end up in the same banquet.
Now, your task involves answering Q queries, each presenting a potential new relationship between two individuals, X and Y. The burning question: If this new connection happens, can you still invite everyone to the party using the two banquets? If the answer is "No", you must figure out the minimum number of relationships to remove so that everyone gets an invite and the number of ways to do so.
NOTE: the queries are independent of each other. Adding a new relationship doesn't permanently change the relationship web.
Input format
    • The first line contains a single integer T, which denotes the number of test cases.
    • For each test case:
        ○ The first line contains N denoting the number of people.
        ○ The following N−1 lines contain 2 space-separated integers, u & v indicating that there is a relationship between person person u & person v
        ○ The next line contains Q denoting the number of queries.
        ○ The next Q lines contain 2 unique space-separated integers, X and Y. It's guaranteed that X and Y initially don't have a relationship.
Output format
For each test case, answer all the Q  queries. For each query print Yes if it's possible to invite everyone to the party using the 2 banquets, after adding the current query's relationship to the initial existing N−1 relations, ensuring that no two people who know each other end up in the same banquet, otherwise print No. If the answer is No, in the next line, print 2 integers, the minimum number of relationships to remove so that everyone gets an invite, and the number of ways to do so.
Constraints
1≤T≤10^5
3≤N≤10^6
1≤u,v≤N
1≤Q≤10^6
1≤X,Y≤N
 The sum of all values of N over all test cases doesn't exceed 10^6
The sum of all values of Q over all test cases doesn't exceed 10^6
*/

struct LCA {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
    log_n: usize,
}

impl LCA {
    fn new(adj: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = adj.len();
        let log_n = (n as f64).log2().ceil() as usize + 1;
        let mut up = vec![vec![n; log_n]; n];
        let mut depth = vec![0; n];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        up[root][0] = root;
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if v != up[u][0] {
                    up[v][0] = u;
                    depth[v] = depth[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        for k in 1..log_n {
            for v in 0..n {
                up[v][k] = up[up[v][k - 1]][k - 1];
            }
        }
        Self { up, depth, log_n }
    }
    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        for k in (0..self.log_n).rev() {
            if self.depth[u] >= (1 << k) && self.depth[u] - (1 << k) >= self.depth[v] {
                u = self.up[u][k];
            }
        }
        if u == v {
            return u;
        }
        for k in (0..self.log_n).rev() {
            if self.up[u][k] != self.up[v][k] {
                u = self.up[u][k];
                v = self.up[v][k];
            }
        }
        self.up[u][0]
    }
    fn path_length(&self, u: usize, v: usize) -> usize {
        let lca = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[lca]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let mut adj = vec![vec![]; n];
        for _ in 0..n - 1 {
            let line = lines.next().unwrap().unwrap();
            let mut it = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap() - 1);
            let u = it.next().unwrap();
            let v = it.next().unwrap();
            adj[u].push(v);
            adj[v].push(u);
        }
        let lca = LCA::new(&adj, 0);

        let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let mut it = line
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap() - 1);
            let x = it.next().unwrap();
            let y = it.next().unwrap();

            let path_len = lca.path_length(x, y);
            let cycle_len = path_len + 1;

            if cycle_len % 2 == 0 {
                println!("Yes");
            } else {
                println!("No");
                println!("1 {}", cycle_len);
            }
        }
    }
}
