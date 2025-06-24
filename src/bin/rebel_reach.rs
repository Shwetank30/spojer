/**
 * Problem
In a kingdom with N cities connected by N−1 roads, the king's palace is located in the first city. Each city has a specific number of guards, given in an array Guards[], where Guards[i] is the number of guards in the city i. The kingdom must assess the risk of revolts starting from any city and how close rebels could get to the king's palace. The guards' effectiveness is such that one guard can stop one rebel.
To tackle this, the king has asked you to develop a method for analyzing such scenarios. This includes answering queries of type 'query(U, X)', where U represents the city where the revolt might begin, and X indicates the number of protestors at the start. Your task is to determine for each query the nearest city to the king's palace (i.e., city 1) that the rebels can reach.
Note: If the rebels can reach City 1, you should print City 1, as this is the city where the King's palace is located.
Input format
    • The first line contains a single integer T, which denotes the number of test cases.
    • For each test case:
        ○ The first line contains N denoting the number of Cities.
        ○ The following N−1 lines contain 2 space-separated integers, u & v indicating that there is a road between City u & City v
        ○ The next line contains N integers, denoting the array Guards.
        ○ The next line contains Q denoting the number of queries.
        ○ The next Q lines contain 2 space-separated integers, U and X, where U denotes the city where the revolt might begin, and X indicates the number of rebels at the start.
Output format
For each test case, answer all the Q  queries. For each query, print in a new line the nearest city to the king's palace (i.e., city 1) that the rebels can reach.
Constraints
1≤T≤10^5
3≤N≤10^6
1≤u,v≤N
1≤Guards[i]≤10^9 ∀ i∈[1,N]
1≤Q≤10^6
1≤U≤N
1≤X≤10^15
The sum of all values of N over all test cases doesn't exceed 10^6
The sum of all values of Q over all test cases doesn't exceed 10^6
Sample Input
1
7
1 2
1 5
2 3
5 6
5 7
3 4
5 3 2 4 6 4 4
3
4 10
6 10
7 2
Sample Output
1
5
7

*/

fn main() {
    
}
