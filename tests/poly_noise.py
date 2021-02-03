import maya.standalone as standalone

standalone.initialize()
import maya.cmds as cmds
from maya_poly_noise import poly_noise

sphere = cmds.polySphere(radius=1, subdivisionsX=200, subdivisionsY=200)
poly_noise(sphere[0], 100)
