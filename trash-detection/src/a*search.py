from __future__ import print_function
import heapq
import matplotlib.pyplot as plt
from planar import Polygon
import math
import itertools




##A new function, which given 4 parameters of 2 points (x1, y1) , (x2, y2), creates a line connecting them (barrier)
def get_line(x1, y1, x2, y2):
    points = []
    issteep = abs(y2-y1) > abs(x2-x1)
    if issteep:
        x1, y1 = y1, x1
        x2, y2 = y2, x2
    rev = False
    if x1 > x2:
        x1, x2 = x2, x1
        y1, y2 = y2, y1
        rev = True
    deltax = x2 - x1
    deltay = abs(y2-y1)
    error = int(deltax / 2)
    y = y1
    ystep = None
    if y1 < y2:
        ystep = 1
    else:
        ystep = -1
    for x in range(x1, x2 + 1):
        if issteep:
            points.append((y, x))
        else:
            points.append((x, y))
        error -= deltay
        if error < 0:
            y += ystep
            error += deltax
    # Reverse the list if the coordinates were reversed
    if rev:
        points.reverse()
    return points

##The Graph, which will include the barrier polygons
class Graph(object):
   
    
    def __init__(self):
        self.barriers = []
        coord = [(1,1), (3,1), (3,5), (1,5)]
        arr = []
        arr2 = []
        arr3 = []
        arr4 = []
        x1 = coord[0][0]
        y1 = coord[0][1]
        x2 = coord[1][0]
        y2 = coord[1][1]
        x3 = coord[2][0]
        y3 = coord[2][1]
        x4 = coord[3][0]
        y4 = coord[3][1]
        arr = get_line(x1, y1, x2, y2)
        arr2 = get_line(x2,y2,x3,y3)
        arr3 = get_line(x3,y3,x4,y4)
        arr4 = get_line(x4,y4,x1,y1)
        
        
        #for 
        ##print(self.intermediates(coord[1], coord[2], 2))
        #coord.append(self.intermediates(coord[1], coord[2], nb_points=3))
        self.barriers.append(coord)
        self.barriers.append(arr)
        self.barriers.append(arr2)
        self.barriers.append(arr3)
        self.barriers.append(arr4)


    def heuristic(self, start, goal):
        #Use Chebyshev distance heuristic if we can move one square either
        #adjacent or diagonal
        D = 1
        D2 = 1
        dx = abs(start[0] - goal[0])
        dy = abs(start[1] - goal[1])
        return D * (dx + dy) + (D2 - 2 * D) * min(dx, dy)

    def get_vertex_neighbours(self, pos):
        n = []
        #Moves allow link a chess king
        for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1),
                       (1, -1), (-1, -1)]:
            x2 = pos[0] + dx
            y2 = pos[1] + dy
            if x2 < 0 or x2 > 7 or y2 < 0 or y2 > 7:
                continue
            n.append((x2, y2))
        return n

    def move_cost(self, a, b):
        for barrier in self.barriers:
            if b in barrier:
                ##print("hit a barrier")
                return 100  #Extremely high cost to enter barrier squares
        return 1  #Normal movement cost


##This will probably not be used, as we A* algorithm works better and is more useful in this context.
def dijkstra(graph, start_vertex):
    distances = {vertex: float('infinity') for vertex in graph}
    distances[start_vertex] = 0
    pq = [(0, start_vertex)]
    while len(pq) > 0:
        current_distance, current_vertex = heapq.heappop(pq)
        if current_distance > distances[current_vertex]:
            continue
        for neighbor, weight in graph[current_vertex].items():
            distance = current_distance + weight
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heapq.heappush(pq, (distance, neighbor))
    return distances


##This is the a* algorithm and the most important part of my code, takes 3 parameters(start, end, graph)
##start --> initial position of boat/drone
##end --> position of trash
##graph --> see Graph class
def asearch(start, end, graph):
    G = {}
    F = {}
    G[start] = 0
    F[start] = graph.heuristic(start, end)
    closedVertices = set()
    openVertices = set([start])
    cameFrom = {}
    while len(openVertices) > 0:
        current = None
        currentFscore = 1
        for pos in openVertices:
            if current is None or F[pos] < currentFscore:
                currentFscore = F[pos]
                current = pos
        if current == end:
            path = [current]
            while current in cameFrom:
                current = cameFrom[current]
                path.append(current)
            path.reverse()
            return path, F[end]
        openVertices.remove(current)
        closedVertices.add(current)
        for neighbor in graph.get_vertex_neighbours(current):
            if neighbor in closedVertices:
                continue
            candidateG = G[current] + graph.move_cost(current, neighbor)
            if neighbor not in openVertices:
                openVertices.add(neighbor)
            elif candidateG >= G[neighbor]:
                continue
            cameFrom[neighbor] = current
            G[neighbor] = candidateG
            H = graph.heuristic(neighbor, end)
            F[neighbor] = G[neighbor] + H
    raise RuntimeError("A* failed to find a solution")


##Just for testing purposes, no need to use this
G = {
    's': {
        'u': 10,
        'x': 5
    },
    'u': {
        'v': 1,
        'x': 2
    },
    'v': {
        'y': 4
    },
    'x': {
        'u': 3,
        'v': 9,
        'y': 2
    },
    'y': {
        's': 7,
        'v': 6
    }
}
print(dijkstra(G, 's'))
graph = Graph()
result, cost = asearch((0, 0), (4,4), graph)
print("route", result)
print("cost", cost)
print("barriers", graph.barriers)
for barrier in graph.barriers:
    plt.plot([v[0] for v in barrier], [v[1] for v in barrier])
plt.plot([v[0] for v in result], [v[1] for v in result])
plt.xlim(-1, 8)
plt.ylim(-1, 8)
plt.show()


