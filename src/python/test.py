import animotion

scene = animotion.create_scene([1.0, 0.5, 0.5, 1.0], 30)
line = animotion.Line(
    start_point=(0, 0, 1),
    end_point=(0.5, 0.5, 1),
    start_color=(1, 0, 0, 1),
    end_color=(0, 1, 0, 1),
    thickness=1.0,
)
line2 = animotion.Line(
    start_point=(0, 0, 1),
    end_point=(-0.5, 0.5, 1),
    start_color=(1, 0, 0, 1),
    end_color=(0, 1, 0, 1),
    thickness=1.0,
)
tetra = animotion.Tetrahedron(
    (0, 0, 0),
    size=1,
    color=([0, 0, 1, 1], [0, 1, 0, 1], [1, 0, 0, 1], [0, 0, 0, 1]),
)
scene.add_object(line)
scene.add_object(line2)
# scene.add_object(tetra)
#
sphere = animotion.Sphere([0.0, 0.0, 0.0], [1.0, 0.0, 0.2, 1.0], 0.2, 50)
scene.add_object(sphere)

animotion.debug_window(scene)

print("Succeeded")
