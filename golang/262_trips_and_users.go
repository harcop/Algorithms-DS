package leetcode

// LeetCode #262 - Trips and Users
//
// Cancellation rate is the share of unfinished trips by unbanned clients on each day.
const tripsAndUsersSQL = "SELECT t.Request_at AS Day,\n" +
	"  ROUND(SUM(IF(t.Status != 'completed', 1, 0)) / COUNT(*), 2) AS `Cancellation Rate`\n" +
	"FROM Trips t\n" +
	"JOIN Users u ON t.Client_Id = u.Users_Id AND u.Banned = 'No'\n" +
	"WHERE t.Request_at BETWEEN '2013-10-01' AND '2013-10-03'\n" +
	"GROUP BY t.Request_at"
