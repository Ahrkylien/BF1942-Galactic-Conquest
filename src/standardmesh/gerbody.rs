subshader "Japbody_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.1 0.1 0.1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	texture "texture/snowtrooper/body.dds";
}
subshader "Japbody_Material1" "StandardMesh/Default"
{
	llighting true;
	lightingSpecular false;
	materialDiffuse 0.1 0.1 0.1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	twosided true;
	texture "texture/snowtrooper/body.dds";
}
