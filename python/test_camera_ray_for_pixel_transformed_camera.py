import numpy as np
from numpy.linalg import inv
from numpy.linalg import norm

print("numpy version", np.version.version)

pixel_size = 0.0099502487562189035;
camera_rot_y = np.array([[0.70710678118654757, 0.0, 0.70710678118654757, 0.0],
                         [0.0, 1.0, 0.0, 0.0],
                         [-0.70710678118654757, 0.0, 0.70710678118654757, 0.0],
                         [0.0, 0.0, 0.0, 1.0]]);

camera_translate = np.array([[1.0, 0.0, 0.0, 0.0],
                             [0.0, 1.0, 0.0, -2.0],
                             [0.0, 0.0, 1.0, 5.0],
                             [0.0, 0.0, 0.0, 1.0]]);

camera_transform = np.dot(camera_rot_y, camera_translate)

print ("pixelsize = ", pixel_size)
print ("camera_rot_y = ", camera_rot_y)
print("camera_tranlaste = ", camera_translate)
print("camera_transform = ", camera_transform)

x = 100
y = 50

x_offset = (x + 0.5) * pixel_size;
y_offset = (y + 0.5) * pixel_size;

print ("x_offset = ", x_offset)
print ("y_offset = ", y_offset)

half_width = 0.99999999999999988
half_height = 0.50248756218905466
world_x = half_width - x_offset
world_y = half_height - y_offset

print ("world_x = ", world_x)
print ("world_y = ", world_y)

camera_transform_inv = inv(camera_transform)
print ("camera_transform_inv = ", camera_transform_inv)

p = np.array([world_x, world_y, -1.0, 0.0]);
print ("p = ", p)

pixel = camera_transform_inv.dot(p);

print ("pixel = ", pixel)

zero_point = np.array([0, 0.0, 0.0, 0.0]);

origin = camera_transform_inv.dot(zero_point);
print ("origin = ", origin)

direction = (pixel - origin) / norm(pixel - origin)

print ("direction = ", direction)
