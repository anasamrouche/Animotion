import animotion

scene = animotion.create_scene([1.0, 0.5, 0.5, 1.0])
line = animotion.Line(
    start_point=(0, 0, 1),
    end_point=(0.5, 0.5, 1),
    start_color=(1, 0, 0, 1),
    end_color=(0, 1, 0, 1),
    thickness=3.0,
)
line2 = animotion.Line(
    start_point=(0, 0, 1),
    end_point=(-0.5, 0.5, 1),
    start_color=(1, 0, 0, 1),
    end_color=(0, 1, 0, 1),
    thickness=3.0,
)
scene.add_object(line)
scene.add_object(line2)

animotion.debug_window(scene)

print("Succeeded")
