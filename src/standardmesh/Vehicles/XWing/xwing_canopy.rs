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
	materialDiffuse 0.0 0.0 0.0;
	transparent true;
	twosided true;
      depthWrite false;
	texture "texture/Vehicles/XWing/xwing_glass";
}

