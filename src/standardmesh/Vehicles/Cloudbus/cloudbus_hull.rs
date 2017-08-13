subshader "cloudbus_hull_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular true;
	materialSpecular 0.2 0.2 0.2;
	materialSpecularPower 4.0;
	envmap true;
	twosided true;
	texture "texture/Vehicles/Cloudbus/cloudbus1_v2";
}

subshader "cloudbus_hull_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular true;
	materialSpecular 0.2 0.2 0.2;
	materialSpecularPower 4.0;
	envmap true;
	twosided true;
	transparent true;
	depthwrite true;
	texture "texture/Vehicles/Cloudbus/cloudbus1_v2";
}