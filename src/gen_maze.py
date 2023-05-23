import random

def get_node_list(maze):
    node_list = []
    for i in range(len(maze)):
        for j in range(len(maze[i])):
            node_list.append(((i, j), maze[i][j]))
    return node_list

height = 10
width = 10

maze = []

for i in range(height):
    maze.append([])
    for j in range(width):
        maze[i].append(random.randint(0, 100))

def prim(maze):
    maze = get_node_list(maze)

    S1 = []
    S2 = []

    S1.append(random.choice(maze))
    maze.remove(S1[0])
    S2 = maze

    E = []
    for i in S2:
        E.append(i)



prim(maze)
