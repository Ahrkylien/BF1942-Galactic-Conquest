subshader "xwing_fus_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
      	depthWrite false;
	texture "texture/Vehicles/XWing/XWing_Fus";
}

subshader "xwing_canopy_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.0 0.0 0.0;
	materialSpecularPower 0.0;
	envmap true;
	twosided false;
      	depthWrite true;
	texture "texture/Vehicles/XWing/xwing_glass";
}

