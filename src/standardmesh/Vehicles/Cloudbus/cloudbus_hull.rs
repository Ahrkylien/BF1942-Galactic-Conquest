subshader "cloudbus_hull_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	twosided true;
	texture "texture/Vehicles/Cloudbus/cloudbus1_v2";
}

subshader "cloudbus_hull_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	twosided true;
	envmap true;
	depthwrite true;
	texture "texture/Vehicles/Cloudbus/cloudbus1_v2";
}

