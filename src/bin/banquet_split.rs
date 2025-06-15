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

fn main() {
    
} 